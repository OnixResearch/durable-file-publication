# Durability Acknowledgment Specification

## Purpose

Define an honest durability-acknowledgment contract for Durable File Publication so a caller never receives an affirmative durable acknowledgment that was not reached.

## ADDED Requirements

### Requirement: The decision names the durability disposition

r[durable_file_publication.durability.decision]
The publication MUST return one named verdict: an affirmative durable acknowledgment, an unknown durability disposition, or a bounded failure.

#### Scenario: State is reached

- GIVEN a caller declares that the intermediate durability state is reached
- WHEN the core classifies the publication
- THEN the verdict MUST be an affirmative durable acknowledgment

#### Scenario: Durability cannot be confirmed

- GIVEN a rename succeeds but the intermediate durability state is not confirmed
- WHEN the core classifies the publication
- THEN the verdict MUST be `CommittedDurabilityUnknown`
- AND it MUST NOT be an affirmative durable acknowledgment

### Requirement: No affirmative ack for an unconfirmed state

r[durable_file_publication.durability.ack]
The shell MUST return an affirmative durable acknowledgment only after a durability intermediate state is reached. It MUST NOT fabricate one.

#### Scenario: Synchronization fails

- GIVEN parent synchronization fails after a rename
- WHEN the shell reports the publication
- THEN the disposition MUST be `CommittedDurabilityUnknown`
- AND no affirmative durable acknowledgment MUST be returned

#### Scenario: Race prevents confirmation

- GIVEN a race prevents the shell from confirming durability
- WHEN the shell reports the publication
- THEN the disposition MUST be `CommittedDurabilityUnknown`
- AND no affirmative durable acknowledgment MUST be returned

### Requirement: Claims stay ownership-bound

r[durable_file_publication.durability.boundary]
The crate MUST NOT claim retention, recovery, deployment, or deletion authority. The caller owns the medium and lifecycle.

#### Scenario: Caller over-claims

- GIVEN a caller or documentation claims retention, recovery, deployment, or deletion support
- WHEN ownership and boundary verification runs
- THEN that claim MUST fail verification
- AND the caller-owned boundary MUST remain explicit

### Requirement: Failure coverage remains explicit

r[durable_file_publication.durability.verification]
Positive and negative fixtures MUST cover every declared disposition and boundary.

#### Scenario: Complete focused matrix passes

- GIVEN decision, shell, fixtures, documentation, and dependency evidence are complete
- WHEN focused no-std, Wasm, package, workspace, Clippy, Cairn, and Nix verification runs
- THEN the reached-state input MUST produce an affirmative ack
- AND each failed, unconfirmed, raced, or overclaiming input MUST resolve to the declared disposition or failure
