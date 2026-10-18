# Durable File Publication Specification

## ADDED Requirements

### Requirement: Repository identity and provenance are independent

r[durable_publication.repository.identity] The repository MUST expose one independently versioned product-neutral crate, MUST bind promoted source files to reviewed Artifact revision `c932138d880ddf4c2967f4c024b489b5c0022bf1`, and MUST NOT depend on Artifact, Mantle, OnixOS, Valence, Cairn, or another stack product at runtime.

#### Scenario: Reviewed fixture is promoted
- GIVEN the clean-room Artifact candidate and its approved extraction decision
- WHEN source provenance is recorded
- THEN each promoted source file MUST have a BLAKE3 identity and the crate MUST retain `MIT OR Apache-2.0` terms.

#### Scenario: Product implementation text is proposed
- GIVEN source expression comes from a product repository without a compatible grant
- WHEN provenance validation runs
- THEN release MUST fail even when behavior tests pass.

### Requirement: Scope remains one regular file

r[durable_publication.scope.one_file] One operation MUST publish one newly staged regular file to one admitted destination leaf beneath one caller-opened parent, and MUST exclude ambient roots, symlink payloads, directory trees, multiple destinations, generation pointers, and transaction markers.

#### Scenario: One bounded payload targets one leaf
- GIVEN one caller-opened parent, one admitted leaf, and one bounded payload
- WHEN publication runs
- THEN all mutation MUST stay relative to that parent and one rename MAY commit that destination.

#### Scenario: Caller requests multiple destinations
- GIVEN a payload and sidecar must commit together
- WHEN request admission runs
- THEN the caller MUST use a product-owned transaction instead of this mechanism.

### Requirement: Request and phase logic stays pure

r[durable_publication.core.classification] The `no_std + alloc` core MUST validate explicit request facts and classify finite ordered observations without filesystem, environment, process, network, clock, randomness, serialization, presentation, or ambient mutable state.

#### Scenario: Valid durable observations complete
- GIVEN ordered stage, write, permission, payload-sync, rename, and parent-sync observations
- WHEN classification runs
- THEN it MUST return `CommittedAndParentSynchronized` deterministically.

#### Scenario: Impossible order is supplied
- GIVEN parent synchronization appears before rename or an observation follows a terminal result
- WHEN classification runs
- THEN it MUST return a stable invalid-transition error.

### Requirement: Admission and resource bounds precede effects

r[durable_publication.shell.preflight] The shell MUST validate leaf safety, exact payload length, payload maximum, collision-attempt limit, permission bits, replacement support, and parent-sync support before stage creation where those facts are available.

#### Scenario: Payload exceeds its bound
- GIVEN payload bytes exceed the explicit maximum
- WHEN publication runs
- THEN it MUST reject before requesting a stage name or performing I/O.

#### Scenario: Required primitive is unavailable
- GIVEN no-replace or required parent synchronization lacks an exact primitive
- WHEN preflight runs
- THEN it MUST return unsupported before stage creation.

### Requirement: Staging is private and bounded

r[durable_publication.shell.staging] The shell MUST use exclusive no-follow regular-file creation, injected stage leaves, a finite collision limit, complete writes, explicit final Unix permissions, and staged-file synchronization before rename.

#### Scenario: Safe stage becomes available
- GIVEN earlier injected leaves collide and one later leaf is available within the bound
- WHEN staging runs
- THEN the shell MUST create only the available private stage and continue in the required order.

#### Scenario: Symlink occupies a stage leaf
- GIVEN a generated stage leaf names an existing symlink
- WHEN exclusive no-follow creation runs
- THEN the symlink MUST remain unchanged and the shell MUST treat the name as a collision or fail safely.

### Requirement: Replacement behavior is exact

r[durable_publication.shell.replacement] Every request MUST select replace or no-replace. Replace MUST use one same-parent rename. No-replace MUST use one race-free platform primitive or return unsupported before mutation.

#### Scenario: Destination races with no-replace
- GIVEN another actor creates the destination before rename
- WHEN the no-replace primitive runs
- THEN the destination MUST remain unchanged and the result MUST be `DestinationExists` with explicit stage cleanup state.

#### Scenario: Only inspection and replacing rename exist
- GIVEN a platform lacks atomic no-replace
- WHEN no-replace is requested
- THEN the shell MUST NOT inspect and then use replacing rename.

### Requirement: Visibility and durability remain distinct

r[durable_publication.shell.durability] The shell MUST synchronize payload data and metadata before rename. Durability-required success MUST also require successful parent synchronization after rename, while visibility-only completion MUST carry an explicit durability non-claim.

#### Scenario: Parent synchronization succeeds
- GIVEN rename commits and required parent synchronization succeeds
- WHEN classification completes
- THEN the result MUST be `CommittedAndParentSynchronized`.

#### Scenario: Visibility-only publication commits
- GIVEN the request does not require parent synchronization
- WHEN rename commits
- THEN the result MUST be `CommittedVisibilityOnly` and MUST NOT imply crash durability.

### Requirement: Partial commits and cleanup cannot be hidden

r[durable_publication.outcome.commit_state] Outcomes MUST preserve whether rename committed. Parent-sync failure after rename MUST produce `CommittedDurabilityUnknown`, and a pre-rename primary failure MUST preserve cleanup success or failure separately.

#### Scenario: Parent synchronization fails after rename
- GIVEN the destination became visible and parent synchronization fails
- WHEN the shell returns
- THEN it MUST report a committed destination with unknown durability and MUST NOT attempt rollback.

#### Scenario: Write and cleanup both fail
- GIVEN payload writing fails before rename and stage removal also fails
- WHEN the shell returns
- THEN it MUST preserve the write failure and failed cleanup without claiming destination commit.

### Requirement: Platform claims are explicit

r[durable_publication.platform.support] The built-in filesystem shell MUST be available only where its exact primitives are implemented, and release evidence MUST distinguish compiled support from executed integration evidence.

#### Scenario: Linux integration executes
- GIVEN a Linux directory file supports exact creation, rename, and synchronization primitives
- WHEN integration tests run
- THEN they MUST check payload bytes, permission bits, destination preservation, and parent-sync completion.

#### Scenario: Unsupported target builds the core
- GIVEN a target lacks the built-in filesystem shell
- WHEN the crate builds without default features
- THEN the pure core MAY build but the repository MUST NOT claim filesystem support.

### Requirement: Conformance includes positive and negative evidence

r[durable_publication.validation.corpus] One versioned corpus MUST cover successful replace and no-replace, malformed input, bounds, unsupported primitives, collisions, every operation-phase failure, cleanup failure, destination races, and post-rename parent-sync failure.

#### Scenario: Corpus reaches every terminal class
- GIVEN the shared corpus runs through the core and fault-injected shell
- WHEN results are compared
- THEN every row MUST retain its expected commit and cleanup state.

#### Scenario: A required negative row is lost
- GIVEN a refactor drops partial-commit, cleanup-failure, or unsupported-platform evidence
- WHEN release validation runs
- THEN release MUST fail.

### Requirement: Product authority remains outside the crate

r[durable_publication.boundary.non_authority] The crate MUST NOT own root authorization, path policy, payload meaning, canonical encoding, content identity, existing-content equivalence, locks, multi-file atomicity, receipts, rollback, recovery, retention, deployment, garbage collection, deletion, or release decisions.

#### Scenario: Existing immutable bytes are identical
- GIVEN no-replace reports `DestinationExists`
- WHEN a consumer compares existing bytes
- THEN only that consumer MAY classify the result as idempotent success.

#### Scenario: Mechanical durability completes
- GIVEN the shell reports parent synchronization success
- WHEN a product evaluates completion
- THEN the product MUST still apply its authority, evidence, lifecycle, and release gates.

### Requirement: Release identity is reproducible and bounded

r[durable_publication.release.evidence] Typed Nickel MUST define repository and release contracts, JSON MUST be a deterministic projection, Nix MUST check formatting, tests, strict Clippy, `no_std` targets, provenance, Cairn, and source projections, and release documentation MUST retain explicit non-claims.

#### Scenario: All selected checks pass
- GIVEN source, contracts, lifecycle artifacts, and projections agree
- WHEN release validation runs
- THEN it MAY publish one immutable Git revision with exact bounded evidence.

#### Scenario: Unsupported platform was not executed
- GIVEN Android or another target did not run integration tests
- WHEN evidence is written
- THEN the platform MUST remain recorded as compiled-only or unsupported instead of passing.
