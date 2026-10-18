# IO Fault Fixtures Specification

## Purpose

Give Durable File Publication deterministic, annotated, test-only injection of `io::Error` so every declared failure path of the publication shell is provable on demand.

## Requirements

### Requirement: Injection is test-only and pinned [r[durable_file_publication.io_fault.fixtures]]

The repository MUST confine `fault-injection` to dev-dependencies of the shell test target. No runtime dependency edge MUST import it. The pinned revision MUST be recorded in the dependency catalog as transport `crates.io`, plane `validation`.

#### Scenario: Runtime code stays unaware

- GIVEN the workspace dependency graph after adoption
- WHEN dependency classification runs
- THEN `fault-injection` MUST appear only on dev edges
- AND the catalog MUST record transport `crates.io` and plane `validation`

#### Scenario: Shell sites are wrapped

- GIVEN the publication shell's staging, rename, file-sync, and parent-sync call sites
- WHEN the test target builds
- THEN each site MUST pass through `fallible!` so a zeroed counter returns an `io::Error`

### Requirement: Injection is deterministic [r[durable_file_publication.io_fault.determinism]]

Fixtures MUST store an explicit counter value before acting and MUST restore the default afterward. `SLEEPINESS` MUST stay zero in every fixture and in CI.

#### Scenario: Fixture fires at a chosen site

- GIVEN a fixture stores a counter value that reaches zero at the parent-sync site
- WHEN publication runs
- THEN the parent-sync call MUST return an injected `io::Error`
- AND the outcome MUST classify as declared

#### Scenario: Random delays stay disabled

- GIVEN any fixture or CI configuration
- WHEN the test suite runs
- THEN `SLEEPINESS` MUST be zero

### Requirement: Unit injection claims nothing beyond itself [r[durable_file_publication.io_fault.boundary]]

Documentation MUST state that unit-level injection does not equal a ChaosControl filesystem campaign and MUST NOT weaken the existing `CommittedDurabilityUnknown` semantics.

#### Scenario: Over-claim rejected

- GIVEN documentation claims campaign-level filesystem-fault equivalence from unit injection
- WHEN boundary verification runs
- THEN the claim MUST fail verification

### Requirement: Failure paths stay covered [r[durable_file_publication.io_fault.verification]]

Positive, negative, and boundary fixtures MUST cover every wrapped site and the annotation contract.

#### Scenario: Complete matrix passes

- GIVEN fixtures for the unchanged path, each injected site, and annotation boundaries
- WHEN package, workspace, Clippy, Cairn, and Nix checks run
- THEN each injected site MUST produce its declared classification
- AND the unchanged path MUST publish as before
