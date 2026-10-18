#![cfg(feature = "std")]
//! Fault-injected positive and negative conformance checks.

#[path = "support/pilots.rs"]
mod pilots;

use durable_file_publication::core::{
    Classification, ClassificationError, CleanupDisposition, DurabilityMode, PhaseObservation,
    PrimaryFailure, PublicationDisposition, PublicationRequest, ReplacementMode,
    classify_publication,
};
use durable_file_publication::corpus::{
    CONFORMANCE_CORPUS, CorpusCase, ExpectedDisposition, FaultPoint,
};
use durable_file_publication::shell::{
    MechanicalIoFailure, PublicationIo, RenameDisposition, StageCreateDisposition, StageNameSource,
    publish_one_file,
};
use pilots::{
    ExistingContent, MANTLE_REVISION, MantlePilotDisposition, MantlePilotRequest, ONIXOS_REVISION,
    OnixOverwritePolicy, OnixPilotRequest, map_mantle_outcome, map_mantle_request,
    map_onix_outcome, map_onix_request,
};

const PAYLOAD: &[u8] = b"bounded-publication-payload";
const PAYLOAD_LIMIT: usize = 128;
const OVER_BOUND_LIMIT: usize = 1;
const COLLISION_LIMIT: usize = 3;
const FINAL_MODE: u32 = 0o640;
const PRIVATE_MODE_FALLBACK: u32 = 0o600;
const INVALID_PERMISSION_MODE: u32 = 0o1_000;
const EXPECTED_REVISION_BYTES: usize = 40;

#[derive(Clone, Debug)]
struct SequenceNames {
    next: usize,
    unsafe_name: bool,
    explicit: Vec<String>,
    calls: usize,
}

impl SequenceNames {
    fn generated(unsafe_name: bool) -> Self {
        Self {
            next: 0,
            unsafe_name,
            explicit: Vec::new(),
            calls: 0,
        }
    }
}

impl StageNameSource for SequenceNames {
    fn next_stage_leaf(&mut self) -> String {
        self.calls = self.calls.saturating_add(1);
        if self.unsafe_name {
            return String::from("../unsafe-stage");
        }
        if let Some(value) = self.explicit.get(self.next).cloned() {
            self.next = self.next.saturating_add(1);
            return value;
        }
        let value = format!(".publication-stage-{}", self.next);
        self.next = self.next.saturating_add(1);
        value
    }
}

#[derive(Clone, Debug)]
struct FakeIo {
    no_replace_supported: bool,
    parent_sync_supported: bool,
    destination_exists: bool,
    collisions_remaining: usize,
    fault: FaultPoint,
    cleanup_fails: bool,
    stage_exists: bool,
    create_calls: usize,
    destination_committed: bool,
    events: Vec<&'static str>,
}

impl FakeIo {
    fn from_case(case: CorpusCase) -> Self {
        Self {
            no_replace_supported: case.no_replace_supported,
            parent_sync_supported: case.parent_sync_supported,
            destination_exists: case.destination_exists,
            collisions_remaining: case.collisions_before_success,
            fault: case.fault,
            cleanup_fails: case.cleanup_fails,
            stage_exists: false,
            create_calls: 0,
            destination_committed: false,
            events: Vec::new(),
        }
    }
}

impl PublicationIo for FakeIo {
    fn supports_no_replace(&self) -> bool {
        self.no_replace_supported
    }

    fn supports_parent_sync(&self) -> bool {
        self.parent_sync_supported
    }

    fn create_stage(
        &mut self,
        _leaf: &str,
        _private_mode: u32,
    ) -> Result<StageCreateDisposition, MechanicalIoFailure> {
        self.events.push("create");
        self.create_calls = self.create_calls.saturating_add(1);
        if self.fault == FaultPoint::StageCreate {
            return Err(MechanicalIoFailure);
        }
        if self.collisions_remaining > 0 {
            self.collisions_remaining = self.collisions_remaining.saturating_sub(1);
            return Ok(StageCreateDisposition::AlreadyExists);
        }
        self.stage_exists = true;
        Ok(StageCreateDisposition::Created)
    }

    fn write_all(&mut self, _bytes: &[u8]) -> Result<(), MechanicalIoFailure> {
        self.events.push("write");
        if self.fault == FaultPoint::PayloadWrite {
            Err(MechanicalIoFailure)
        } else {
            Ok(())
        }
    }

    fn apply_permissions(&mut self, _mode: u32) -> Result<(), MechanicalIoFailure> {
        self.events.push("permissions");
        if self.fault == FaultPoint::PermissionApply {
            Err(MechanicalIoFailure)
        } else {
            Ok(())
        }
    }

    fn sync_payload(&mut self) -> Result<(), MechanicalIoFailure> {
        self.events.push("payload-sync");
        if self.fault == FaultPoint::PayloadSync {
            Err(MechanicalIoFailure)
        } else {
            Ok(())
        }
    }

    fn rename_stage(
        &mut self,
        _stage_leaf: &str,
        _destination_leaf: &str,
        replacement: ReplacementMode,
    ) -> Result<RenameDisposition, MechanicalIoFailure> {
        self.events.push("rename");
        if self.fault == FaultPoint::Rename {
            return Err(MechanicalIoFailure);
        }
        if replacement == ReplacementMode::NoReplace && self.destination_exists {
            return Ok(RenameDisposition::DestinationExists);
        }
        self.stage_exists = false;
        self.destination_committed = true;
        Ok(RenameDisposition::Committed)
    }

    fn sync_parent(&mut self) -> Result<(), MechanicalIoFailure> {
        self.events.push("parent-sync");
        if self.fault == FaultPoint::ParentSync {
            Err(MechanicalIoFailure)
        } else {
            Ok(())
        }
    }

    fn cleanup_stage(&mut self, _stage_leaf: &str) -> Result<(), MechanicalIoFailure> {
        self.events.push("cleanup");
        if self.cleanup_fails {
            return Err(MechanicalIoFailure);
        }
        self.stage_exists = false;
        Ok(())
    }
}

fn request(case: CorpusCase) -> PublicationRequest {
    let maximum_payload_bytes = if case.payload_over_bound {
        OVER_BOUND_LIMIT
    } else {
        PAYLOAD_LIMIT
    };
    PublicationRequest {
        destination_leaf: String::from("published.bin"),
        payload_bytes: PAYLOAD.len(),
        maximum_payload_bytes,
        collision_attempt_limit: COLLISION_LIMIT,
        final_mode: FINAL_MODE,
        replacement: case.replacement,
        durability: case.durability,
    }
}

fn disposition_matches(
    result: &Result<Classification, ClassificationError>,
    expected: ExpectedDisposition,
) -> bool {
    match (result, expected) {
        (
            Ok(Classification {
                disposition: PublicationDisposition::CommittedVisibilityOnly,
                ..
            }),
            ExpectedDisposition::Visible,
        )
        | (
            Ok(Classification {
                disposition: PublicationDisposition::CommittedAndParentSynchronized,
                ..
            }),
            ExpectedDisposition::Durable,
        )
        | (
            Ok(Classification {
                disposition: PublicationDisposition::DestinationExists { .. },
                ..
            }),
            ExpectedDisposition::DestinationExists,
        )
        | (
            Ok(Classification {
                disposition: PublicationDisposition::CommittedDurabilityUnknown,
                ..
            }),
            ExpectedDisposition::CommittedDurabilityUnknown,
        )
        | (Err(ClassificationError::InvalidRequest(_)), ExpectedDisposition::InvalidRequest)
        | (Err(ClassificationError::InvalidStageLeaf), ExpectedDisposition::InvalidStageName) => {
            true
        }
        (
            Ok(Classification {
                disposition: PublicationDisposition::NotCommitted { failure, .. },
                ..
            }),
            ExpectedDisposition::NotCommitted(expected_failure),
        ) => *failure == expected_failure,
        _ => false,
    }
}

// r[verify durable_publication.validation.corpus]
// r[verify durable_publication.outcome.commit_state]
#[test]
fn shared_corpus_preserves_commit_cleanup_and_preflight_state() {
    for case in CONFORMANCE_CORPUS {
        let mut io = FakeIo::from_case(*case);
        let mut names = SequenceNames::generated(case.unsafe_stage_name);
        let result = publish_one_file(&mut io, &mut names, &request(*case), PAYLOAD);
        assert!(
            disposition_matches(&result, case.expected),
            "case {} produced {result:?}",
            case.id
        );
        if case.cleanup_fails {
            assert!(result.as_ref().is_ok_and(|classification| matches!(
                classification.disposition,
                PublicationDisposition::NotCommitted {
                    cleanup: CleanupDisposition::Failed,
                    ..
                }
            )));
        }
        if matches!(
            case.expected,
            ExpectedDisposition::InvalidRequest
                | ExpectedDisposition::NotCommitted(
                    PrimaryFailure::NoReplaceUnsupported | PrimaryFailure::ParentSyncUnsupported
                )
        ) {
            assert_eq!(io.create_calls, 0, "case {} performed I/O", case.id);
            assert_eq!(names.calls, 0, "case {} requested a stage name", case.id);
        }
        if case.expected == ExpectedDisposition::CommittedDurabilityUnknown {
            assert!(io.destination_committed);
        }
    }
}

// r[verify durable_publication.shell.preflight]
#[test]
fn malformed_requests_and_wrong_payload_length_mutate_nothing() {
    let malformed = PublicationRequest {
        destination_leaf: String::from("../escape"),
        payload_bytes: PAYLOAD.len(),
        maximum_payload_bytes: 0,
        collision_attempt_limit: 0,
        final_mode: INVALID_PERMISSION_MODE,
        replacement: ReplacementMode::Replace,
        durability: DurabilityMode::VisibilityOnly,
    };
    let case = CONFORMANCE_CORPUS[0];
    let mut io = FakeIo::from_case(case);
    let mut names = SequenceNames::generated(false);
    let malformed_result = publish_one_file(&mut io, &mut names, &malformed, PAYLOAD);
    assert!(matches!(
        malformed_result,
        Err(ClassificationError::InvalidRequest(_))
    ));
    assert_eq!(io.create_calls, 0);
    assert_eq!(names.calls, 0);

    let valid = request(case);
    let wrong_bytes = PublicationRequest {
        payload_bytes: PAYLOAD.len().saturating_add(1),
        ..valid
    };
    let wrong_bytes_result = publish_one_file(&mut io, &mut names, &wrong_bytes, PAYLOAD);
    assert!(matches!(
        wrong_bytes_result,
        Err(ClassificationError::PayloadByteCountMismatch { .. })
    ));
    assert_eq!(io.create_calls, 0);
    assert_eq!(names.calls, 0);
}

// r[verify durable_publication.core.classification]
#[test]
fn impossible_phase_order_and_post_terminal_observations_are_rejected() {
    let durable = PublicationRequest {
        durability: DurabilityMode::DurabilityRequired,
        ..request(CONFORMANCE_CORPUS[0])
    };
    let impossible = classify_publication(&durable, &[PhaseObservation::ParentSynchronized]);
    assert!(matches!(
        impossible,
        Err(ClassificationError::InvalidTransition { .. })
    ));

    let no_replace = PublicationRequest {
        replacement: ReplacementMode::NoReplace,
        durability: DurabilityMode::VisibilityOnly,
        ..request(CONFORMANCE_CORPUS[0])
    };
    let post_terminal = classify_publication(
        &no_replace,
        &[
            PhaseObservation::NoReplaceUnsupported,
            PhaseObservation::CollisionExhausted,
        ],
    );
    assert!(matches!(
        post_terminal,
        Err(ClassificationError::InvalidTransition { .. })
    ));
}

// r[verify durable_publication.shell.durability]
#[test]
fn shell_orders_payload_sync_before_rename_and_parent_sync_after_rename() {
    let case = CONFORMANCE_CORPUS
        .iter()
        .find(|case| case.id == "replace-durable")
        .copied();
    assert!(case.is_some());
    if let Some(case) = case {
        let mut io = FakeIo::from_case(case);
        let mut names = SequenceNames::generated(false);
        let result = publish_one_file(&mut io, &mut names, &request(case), PAYLOAD);
        assert!(result.is_ok());
        assert_eq!(
            io.events,
            [
                "create",
                "write",
                "permissions",
                "payload-sync",
                "rename",
                "parent-sync"
            ]
        );
    }
}

// r[verify durable_publication.boundary.non_authority]
#[test]
fn consumer_pilots_retain_product_policy_outside_the_public_crate() {
    assert_eq!(MANTLE_REVISION.len(), EXPECTED_REVISION_BYTES);
    assert_eq!(ONIXOS_REVISION.len(), EXPECTED_REVISION_BYTES);

    let destination_case = CONFORMANCE_CORPUS
        .iter()
        .find(|case| case.id == "destination-exists")
        .copied();
    assert!(destination_case.is_some());
    if let Some(case) = destination_case {
        let mut io = FakeIo::from_case(case);
        let mut names = SequenceNames::generated(false);
        let classification = publish_one_file(&mut io, &mut names, &request(case), PAYLOAD);
        assert!(classification.is_ok());
        if let Ok(classification) = classification {
            let identical = map_mantle_outcome(&classification, ExistingContent::Identical);
            let different = map_mantle_outcome(&classification, ExistingContent::Different);
            let not_compared = map_mantle_outcome(&classification, ExistingContent::NotCompared);
            assert_eq!(
                identical.product,
                MantlePilotDisposition::IdempotentExisting
            );
            assert_eq!(different.product, MantlePilotDisposition::ImmutableConflict);
            assert_eq!(
                not_compared.product,
                MantlePilotDisposition::ImmutableConflict
            );
            assert_eq!(identical.mechanical, different.mechanical);
            assert_eq!(
                map_onix_outcome(&classification).mechanical,
                classification.disposition
            );
        }
    }

    let mantle = map_mantle_request(&MantlePilotRequest {
        object_leaf: String::from("object.json"),
        payload_bytes: PAYLOAD.len(),
        maximum_payload_bytes: PAYLOAD_LIMIT,
        collision_attempt_limit: COLLISION_LIMIT,
        final_mode: FINAL_MODE,
    });
    assert_eq!(mantle.replacement, ReplacementMode::NoReplace);
    assert_eq!(mantle.durability, DurabilityMode::DurabilityRequired);

    let onix = map_onix_request(&OnixPilotRequest {
        destination_leaf: String::from("state.json"),
        payload_bytes: PAYLOAD.len(),
        maximum_payload_bytes: PAYLOAD_LIMIT,
        collision_attempt_limit: COLLISION_LIMIT,
        exact_mode: None,
        existing_mode: Some(FINAL_MODE),
        create_mode: PRIVATE_MODE_FALLBACK,
        overwrite: OnixOverwritePolicy::Refuse,
        durability_required: false,
    });
    assert_eq!(onix.final_mode, FINAL_MODE);
    assert_eq!(onix.replacement, ReplacementMode::NoReplace);
    assert_eq!(onix.durability, DurabilityMode::VisibilityOnly);

    let replace = map_onix_request(&OnixPilotRequest {
        destination_leaf: String::from("state.json"),
        payload_bytes: PAYLOAD.len(),
        maximum_payload_bytes: PAYLOAD_LIMIT,
        collision_attempt_limit: COLLISION_LIMIT,
        exact_mode: Some(FINAL_MODE),
        existing_mode: None,
        create_mode: PRIVATE_MODE_FALLBACK,
        overwrite: OnixOverwritePolicy::Replace,
        durability_required: true,
    });
    assert_eq!(replace.replacement, ReplacementMode::Replace);
    assert_eq!(replace.durability, DurabilityMode::DurabilityRequired);
}

#[test]
fn parent_sync_failure_is_committed_and_cleanup_failure_stays_precommit() {
    let parent_sync_case = CONFORMANCE_CORPUS
        .iter()
        .find(|case| case.id == "parent-sync-partial-commit")
        .copied();
    assert!(parent_sync_case.is_some());
    if let Some(case) = parent_sync_case {
        let mut io = FakeIo::from_case(case);
        let mut names = SequenceNames::generated(false);
        let result = publish_one_file(&mut io, &mut names, &request(case), PAYLOAD);
        assert!(result.as_ref().is_ok_and(|classification| matches!(
            classification.disposition,
            PublicationDisposition::CommittedDurabilityUnknown
        )));
        assert!(io.destination_committed);
        assert!(!io.events.contains(&"cleanup"));
    }

    let cleanup_case = CONFORMANCE_CORPUS
        .iter()
        .find(|case| case.id == "write-failure-cleanup-failure")
        .copied();
    assert!(cleanup_case.is_some());
    if let Some(case) = cleanup_case {
        let mut io = FakeIo::from_case(case);
        let mut names = SequenceNames::generated(false);
        let result = publish_one_file(&mut io, &mut names, &request(case), PAYLOAD);
        assert!(result.as_ref().is_ok_and(|classification| matches!(
            classification.disposition,
            PublicationDisposition::NotCommitted {
                failure: PrimaryFailure::PayloadWrite,
                cleanup: CleanupDisposition::Failed,
            }
        )));
        assert!(!io.destination_committed);
    }
}
