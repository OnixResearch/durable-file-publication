//! Disposable consumer request and outcome mappings.

use std::string::String;

use durable_file_publication::core::{
    Classification, CleanupDisposition, DurabilityMode, PublicationDisposition, PublicationRequest,
    ReplacementMode,
};

/// Exact Mantle source revision used by this pilot.
pub const MANTLE_REVISION: &str = "688781f370e8f477d900e7b96efa1ad3266a392e";

/// Exact OnixOS source revision used by this pilot.
pub const ONIXOS_REVISION: &str = "628f0b8a7036ed344aa51c74612495e28299c75a";

/// Product-owned existing immutable-object comparison.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExistingContent {
    /// Existing bytes are exactly the admitted Mantle object bytes.
    Identical,
    /// Existing bytes differ from the admitted Mantle object bytes.
    Different,
    /// No comparison was performed.
    NotCompared,
}

/// Minimal Mantle immutable-object pilot input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MantlePilotRequest {
    /// Consumer-owned immutable object leaf.
    pub object_leaf: String,
    /// Exact canonical object bytes.
    pub payload_bytes: usize,
    /// Mantle-owned object byte limit.
    pub maximum_payload_bytes: usize,
    /// Mantle-owned private stage collision limit.
    pub collision_attempt_limit: usize,
    /// Mantle-owned final file mode.
    pub final_mode: u32,
}

/// Mantle-owned interpretation of mechanical output.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MantlePilotDisposition {
    /// New immutable object was durably published.
    Published,
    /// Existing exact bytes make no-replace idempotent for Mantle.
    IdempotentExisting,
    /// Existing bytes conflict with the immutable object identity.
    ImmutableConflict,
    /// No destination commit occurred.
    NotCommitted,
    /// Destination committed, but requested durability is unknown.
    CommittedDurabilityUnknown,
}

/// Lossless Mantle pilot output retains the full mechanical result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MantlePilotOutcome {
    /// Mantle-owned product interpretation.
    pub product: MantlePilotDisposition,
    /// Exact product-neutral disposition retained without narrowing.
    pub mechanical: PublicationDisposition,
}

/// Map Mantle input into the product-neutral candidate request.
#[must_use]
pub fn map_mantle_request(input: &MantlePilotRequest) -> PublicationRequest {
    PublicationRequest {
        destination_leaf: input.object_leaf.clone(),
        payload_bytes: input.payload_bytes,
        maximum_payload_bytes: input.maximum_payload_bytes,
        collision_attempt_limit: input.collision_attempt_limit,
        final_mode: input.final_mode,
        replacement: ReplacementMode::NoReplace,
        durability: DurabilityMode::DurabilityRequired,
    }
}

/// Preserve Mantle's product-owned identical-content rule outside the candidate.
#[must_use]
pub fn map_mantle_outcome(
    classification: &Classification,
    existing: ExistingContent,
) -> MantlePilotOutcome {
    let product = match &classification.disposition {
        PublicationDisposition::CommittedAndParentSynchronized => MantlePilotDisposition::Published,
        PublicationDisposition::DestinationExists { .. } => match existing {
            ExistingContent::Identical => MantlePilotDisposition::IdempotentExisting,
            ExistingContent::Different | ExistingContent::NotCompared => {
                MantlePilotDisposition::ImmutableConflict
            }
        },
        PublicationDisposition::CommittedDurabilityUnknown
        | PublicationDisposition::CommittedVisibilityOnly => {
            MantlePilotDisposition::CommittedDurabilityUnknown
        }
        PublicationDisposition::NotCommitted { .. } => MantlePilotDisposition::NotCommitted,
    };
    MantlePilotOutcome {
        product,
        mechanical: classification.disposition.clone(),
    }
}

/// OnixOS overwrite behavior.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OnixOverwritePolicy {
    /// Replace one existing leaf.
    Replace,
    /// Refuse an existing destination.
    Refuse,
}

/// Minimal OnixOS capability-file pilot input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OnixPilotRequest {
    /// Consumer-admitted destination leaf.
    pub destination_leaf: String,
    /// Exact payload byte count.
    pub payload_bytes: usize,
    /// Adapter-owned explicit byte bound.
    pub maximum_payload_bytes: usize,
    /// Adapter-owned collision bound.
    pub collision_attempt_limit: usize,
    /// Explicit exact mode, when supplied.
    pub exact_mode: Option<u32>,
    /// Existing destination mode, when retained by replace.
    pub existing_mode: Option<u32>,
    /// Explicit create mode fallback.
    pub create_mode: u32,
    /// Product overwrite policy.
    pub overwrite: OnixOverwritePolicy,
    /// Whether the operation requires crash-durability mechanics.
    pub durability_required: bool,
}

/// OnixOS-owned interpretation of candidate output.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OnixPilotDisposition {
    /// Mechanical publication completed at the requested level.
    Published,
    /// Refuse mode observed an existing destination.
    DestinationExists,
    /// No destination commit occurred.
    NotCommitted,
    /// Destination committed, but durability is unknown.
    CommittedDurabilityUnknown,
}

fn onix_final_mode(input: &OnixPilotRequest) -> u32 {
    input
        .exact_mode
        .or(input.existing_mode)
        .unwrap_or(input.create_mode)
}

/// Lossless OnixOS pilot output retains the full mechanical result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OnixPilotOutcome {
    /// OnixOS-owned product interpretation.
    pub product: OnixPilotDisposition,
    /// Exact product-neutral disposition retained without narrowing.
    pub mechanical: PublicationDisposition,
}

/// Map OnixOS capability-file input into the candidate request.
#[must_use]
pub fn map_onix_request(input: &OnixPilotRequest) -> PublicationRequest {
    PublicationRequest {
        destination_leaf: input.destination_leaf.clone(),
        payload_bytes: input.payload_bytes,
        maximum_payload_bytes: input.maximum_payload_bytes,
        collision_attempt_limit: input.collision_attempt_limit,
        final_mode: onix_final_mode(input),
        replacement: match input.overwrite {
            OnixOverwritePolicy::Replace => ReplacementMode::Replace,
            OnixOverwritePolicy::Refuse => ReplacementMode::NoReplace,
        },
        durability: if input.durability_required {
            DurabilityMode::DurabilityRequired
        } else {
            DurabilityMode::VisibilityOnly
        },
    }
}

/// Preserve commit and durability state for OnixOS.
#[must_use]
pub fn map_onix_outcome(classification: &Classification) -> OnixPilotOutcome {
    let product = match &classification.disposition {
        PublicationDisposition::CommittedVisibilityOnly
        | PublicationDisposition::CommittedAndParentSynchronized => OnixPilotDisposition::Published,
        PublicationDisposition::DestinationExists { cleanup } => {
            if *cleanup == CleanupDisposition::Failed {
                OnixPilotDisposition::NotCommitted
            } else {
                OnixPilotDisposition::DestinationExists
            }
        }
        PublicationDisposition::CommittedDurabilityUnknown => {
            OnixPilotDisposition::CommittedDurabilityUnknown
        }
        PublicationDisposition::NotCommitted { .. } => OnixPilotDisposition::NotCommitted,
    };
    OnixPilotOutcome {
        product,
        mechanical: classification.disposition.clone(),
    }
}
