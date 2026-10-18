#![cfg(all(feature = "std", any(target_os = "linux", target_os = "android")))]
//! Linux and Android capability-relative filesystem integration checks.

use std::fs::File;
use std::os::unix::fs::{PermissionsExt, symlink};

use durable_file_publication::core::{
    CleanupDisposition, DurabilityMode, PrimaryFailure, PublicationDisposition, PublicationRequest,
    ReplacementMode,
};
use durable_file_publication::shell::{
    LinuxDirectory, PublicationIo, StageNameSource, publish_one_file,
};
use rustix::fs::{Mode, OFlags, open};

const PAYLOAD: &[u8] = b"bounded-publication-payload";
const REPLACEMENT_PAYLOAD: &[u8] = b"updated-publication-payload";
const PAYLOAD_LIMIT: usize = 128;
const COLLISION_LIMIT: usize = 3;
const FINAL_MODE: u32 = 0o640;
const TEST_PERMISSION_MASK: u32 = 0o777;

#[derive(Clone, Debug)]
struct SequenceNames {
    leaves: Vec<String>,
    next: usize,
}

impl SequenceNames {
    fn new(leaves: &[&str]) -> Self {
        Self {
            leaves: leaves.iter().map(|leaf| String::from(*leaf)).collect(),
            next: 0,
        }
    }
}

impl StageNameSource for SequenceNames {
    fn next_stage_leaf(&mut self) -> String {
        let selected = self
            .leaves
            .get(self.next)
            .cloned()
            .unwrap_or_else(|| format!(".fallback-stage-{}", self.next));
        self.next = self.next.saturating_add(1);
        selected
    }
}

fn request(payload_bytes: usize, replacement: ReplacementMode) -> PublicationRequest {
    PublicationRequest {
        destination_leaf: String::from("published.bin"),
        payload_bytes,
        maximum_payload_bytes: PAYLOAD_LIMIT,
        collision_attempt_limit: COLLISION_LIMIT,
        final_mode: FINAL_MODE,
        replacement,
        durability: DurabilityMode::DurabilityRequired,
    }
}

// r[verify durable_publication.platform.support]
// r[verify durable_publication.shell.staging]
// r[verify durable_publication.shell.replacement]
#[test]
fn open_directory_preserves_symlink_collision_permissions_and_no_replace() {
    let directory = tempfile::tempdir();
    assert!(directory.is_ok());
    let directory = directory.unwrap_or_else(|error| unreachable!("fixture: {error}"));

    let symlink_path = directory.path().join(".occupied-stage");
    let linked = symlink("outside-target", &symlink_path);
    assert!(linked.is_ok());

    let opened = File::open(directory.path());
    assert!(opened.is_ok());
    let opened = opened.unwrap_or_else(|error| unreachable!("fixture: {error}"));
    let parent = LinuxDirectory::from_open_directory(opened);
    assert!(parent.is_ok());
    let mut parent = parent.unwrap_or_else(|error| unreachable!("fixture: {error}"));
    let mut names = SequenceNames::new(&[".occupied-stage", ".available-stage"]);

    let published = publish_one_file(
        &mut parent,
        &mut names,
        &request(PAYLOAD.len(), ReplacementMode::NoReplace),
        PAYLOAD,
    );
    assert!(published.as_ref().is_ok_and(|classification| matches!(
        classification.disposition,
        PublicationDisposition::CommittedAndParentSynchronized
    )));
    let bytes = std::fs::read(directory.path().join("published.bin"));
    assert!(
        bytes
            .as_ref()
            .is_ok_and(|bytes| bytes.as_slice() == PAYLOAD)
    );
    let metadata = std::fs::metadata(directory.path().join("published.bin"));
    assert!(metadata.as_ref().is_ok_and(|metadata| {
        metadata.permissions().mode() & TEST_PERMISSION_MASK == FINAL_MODE
    }));
    let symlink_metadata = std::fs::symlink_metadata(&symlink_path);
    assert!(
        symlink_metadata
            .as_ref()
            .is_ok_and(|metadata| metadata.file_type().is_symlink())
    );

    let mut second_names = SequenceNames::new(&[".second-stage"]);
    let existing = publish_one_file(
        &mut parent,
        &mut second_names,
        &request(PAYLOAD.len(), ReplacementMode::NoReplace),
        PAYLOAD,
    );
    assert!(existing.as_ref().is_ok_and(|classification| matches!(
        classification.disposition,
        PublicationDisposition::DestinationExists {
            cleanup: CleanupDisposition::Removed,
        }
    )));
    let preserved = std::fs::read(directory.path().join("published.bin"));
    assert!(
        preserved
            .as_ref()
            .is_ok_and(|bytes| bytes.as_slice() == PAYLOAD)
    );
}

#[test]
fn durability_rejects_unsynchronizable_parent_before_stage_creation() {
    let directory = tempfile::tempdir();
    assert!(directory.is_ok());
    let directory = directory.unwrap_or_else(|error| unreachable!("fixture: {error}"));
    let descriptor = open(
        directory.path(),
        OFlags::PATH | OFlags::DIRECTORY | OFlags::CLOEXEC,
        Mode::empty(),
    );
    assert!(descriptor.is_ok());
    let descriptor = descriptor.unwrap_or_else(|error| unreachable!("fixture: {error}"));
    let parent = LinuxDirectory::from_open_directory(File::from(descriptor));
    assert!(parent.is_ok());
    let mut parent = parent.unwrap_or_else(|error| unreachable!("fixture: {error}"));
    assert!(!parent.supports_parent_sync());
    let mut names = SequenceNames::new(&[".must-not-exist"]);
    let result = publish_one_file(
        &mut parent,
        &mut names,
        &request(PAYLOAD.len(), ReplacementMode::Replace),
        PAYLOAD,
    );
    assert!(result.as_ref().is_ok_and(|classification| matches!(
        classification.disposition,
        PublicationDisposition::NotCommitted {
            failure: PrimaryFailure::ParentSyncUnsupported,
            cleanup: CleanupDisposition::NotNeeded,
        }
    )));
    assert!(!directory.path().join(".must-not-exist").exists());
}

#[test]
fn replace_commits_new_bytes_and_non_directory_parent_is_rejected() {
    let directory = tempfile::tempdir();
    assert!(directory.is_ok());
    let directory = directory.unwrap_or_else(|error| unreachable!("fixture: {error}"));
    let initial = std::fs::write(directory.path().join("published.bin"), PAYLOAD);
    assert!(initial.is_ok());

    let opened = File::open(directory.path());
    assert!(opened.is_ok());
    let opened = opened.unwrap_or_else(|error| unreachable!("fixture: {error}"));
    let parent = LinuxDirectory::from_open_directory(opened);
    assert!(parent.is_ok());
    let mut parent = parent.unwrap_or_else(|error| unreachable!("fixture: {error}"));
    let mut names = SequenceNames::new(&[".replace-stage"]);
    let replaced = publish_one_file(
        &mut parent,
        &mut names,
        &request(REPLACEMENT_PAYLOAD.len(), ReplacementMode::Replace),
        REPLACEMENT_PAYLOAD,
    );
    assert!(replaced.as_ref().is_ok_and(|classification| matches!(
        classification.disposition,
        PublicationDisposition::CommittedAndParentSynchronized
    )));
    let bytes = std::fs::read(directory.path().join("published.bin"));
    assert!(
        bytes
            .as_ref()
            .is_ok_and(|bytes| bytes.as_slice() == REPLACEMENT_PAYLOAD)
    );

    let regular_file = File::open(directory.path().join("published.bin"));
    assert!(regular_file.is_ok());
    let regular_file = regular_file.unwrap_or_else(|error| unreachable!("fixture: {error}"));
    assert!(LinuxDirectory::from_open_directory(regular_file).is_err());
}
