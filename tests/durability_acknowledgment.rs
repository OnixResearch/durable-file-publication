//! Durability acknowledgment positive, negative, and boundary cases.

use durable_file_publication::core::{
    Classification, CleanupDisposition, DURABILITY_ACKNOWLEDGMENT_NON_CLAIMS,
    DurabilityAcknowledgmentDisposition, IntermediateDurabilityObservation, PrimaryFailure,
    PublicationDisposition, decide_durability_acknowledgment,
};
use durable_file_publication::shell::durability_acknowledgment;

// r[verify durable_file_publication.durability.decision]
// r[verify durable_file_publication.durability.ack]
// r[verify durable_file_publication.durability.boundary]
// r[verify durable_file_publication.durability.verification]
#[test]
fn reached_intermediate_state_produces_the_only_affirmative_ack() {
    let publication = PublicationDisposition::CommittedAndParentSynchronized;
    let decision =
        decide_durability_acknowledgment(&publication, IntermediateDurabilityObservation::Reached);

    assert_eq!(
        decision.disposition,
        DurabilityAcknowledgmentDisposition::CommittedDurable
    );
    assert_eq!(decision.publication, publication);
    assert_eq!(
        decision.required_non_claims,
        DURABILITY_ACKNOWLEDGMENT_NON_CLAIMS
    );
}

#[test]
fn absent_and_unconfirmed_facts_preserve_unknown_durability() {
    for observation in [
        IntermediateDurabilityObservation::Absent,
        IntermediateDurabilityObservation::Unconfirmed,
    ] {
        let decision = decide_durability_acknowledgment(
            &PublicationDisposition::CommittedAndParentSynchronized,
            observation,
        );
        assert_eq!(
            decision.disposition,
            DurabilityAcknowledgmentDisposition::CommittedDurabilityUnknown
        );
    }
}

#[test]
fn failed_synchronization_and_visibility_only_never_ack_durability() {
    for publication in [
        PublicationDisposition::CommittedDurabilityUnknown,
        PublicationDisposition::CommittedVisibilityOnly,
    ] {
        let decision = decide_durability_acknowledgment(
            &publication,
            IntermediateDurabilityObservation::Reached,
        );
        assert_eq!(
            decision.disposition,
            DurabilityAcknowledgmentDisposition::CommittedDurabilityUnknown
        );
    }
}

#[test]
fn failed_rename_and_destination_race_stay_not_committed() {
    for publication in [
        PublicationDisposition::NotCommitted {
            failure: PrimaryFailure::Rename,
            cleanup: CleanupDisposition::Removed,
        },
        PublicationDisposition::DestinationExists {
            cleanup: CleanupDisposition::Removed,
        },
    ] {
        let decision = decide_durability_acknowledgment(
            &publication,
            IntermediateDurabilityObservation::Reached,
        );
        assert_eq!(
            decision.disposition,
            DurabilityAcknowledgmentDisposition::PublicationNotCommitted
        );
        assert_eq!(decision.publication, publication);
    }
}

#[test]
fn shell_ack_surface_delegates_to_the_pure_decision() {
    let classification = Classification {
        disposition: PublicationDisposition::CommittedAndParentSynchronized,
        observations: Vec::new(),
    };
    let shell = durability_acknowledgment(
        &classification,
        IntermediateDurabilityObservation::Unconfirmed,
    );
    let core = decide_durability_acknowledgment(
        &classification.disposition,
        IntermediateDurabilityObservation::Unconfirmed,
    );

    assert_eq!(shell, core);
    assert_eq!(
        shell.disposition,
        DurabilityAcknowledgmentDisposition::CommittedDurabilityUnknown
    );
}

#[test]
fn non_claims_keep_caller_owned_authority_explicit() {
    assert!(
        DURABILITY_ACKNOWLEDGMENT_NON_CLAIMS
            .iter()
            .any(|claim| claim.contains("retention"))
    );
    assert!(
        DURABILITY_ACKNOWLEDGMENT_NON_CLAIMS
            .iter()
            .any(|claim| claim.contains("recovery"))
    );
    assert!(
        DURABILITY_ACKNOWLEDGMENT_NON_CLAIMS
            .iter()
            .any(|claim| claim.contains("deletion"))
    );
    assert!(
        DURABILITY_ACKNOWLEDGMENT_NON_CLAIMS
            .iter()
            .any(|claim| claim.contains("replication"))
    );
}
