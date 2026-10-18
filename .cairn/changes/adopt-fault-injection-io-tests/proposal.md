## Why

Durable File Publication reports bounded durability states for one-file publication, including `CommittedDurabilityUnknown` when synchronization fails. Its negative paths (failed rename, failed file sync, failed parent sync) are the safety-critical half of the contract. Today those paths are covered only where a filesystem error can be provoked naturally, which is irregular and not repeatable.

The komora-io audit (2026-09-09) selected `fault-injection` (Apache-2.0, crates.io) as the stack-wide dev-dependency for deterministic `io::Error` injection with crate, file, and line annotation. This change is the pilot for that adoption. It proves the fixture pattern here first; bounded-exec, bounded-http, bounded-tree, artifact, and mantle reuse the pattern after this change lands.

## What Changes

- Add `fault-injection` as a pinned dev-dependency. It must not appear on any runtime dependency edge.
- Wrap the shell's I/O call sites with the `fallible!` macro in test builds so a decremented `FAULT_INJECT_COUNTER` returns an annotated `io::Error` at a chosen site.
- Add negative fixtures for failed rename, failed file sync, and failed parent sync that classify into the declared bounded failures or `CommittedDurabilityUnknown`.
- Add one positive fixture that proves the counter far from zero leaves publication behavior unchanged.
- Record the dependency in the workspace dependency catalog as transport `crates.io`, plane `validation`.

## Impact

Immediate outcome: every declared failure path of the publication shell becomes testable on demand. Durable capability contribution: a repeatable negative-path fixture pattern that the five follow-on repositories copy. Maintenance owner: this repository. Repeatability evidence: each fixture sets an explicit counter value and asserts the classified outcome; no fixture depends on ambient filesystem state.

## Dependencies

This change has no downstream prerequisite. The five follow-on adoptions in bounded-exec, bounded-http, bounded-tree, artifact, and mantle name this change as their pilot gate.

## Non-goals

- Do not add `fault-injection` to any non-dev dependency edge.
- Do not enable `SLEEPINESS` random delays in any test; they break deterministic classification.
- Do not change publication semantics, decision classification, or receipt shapes.
- Do not claim filesystem-fault equivalence with ChaosControl campaigns; this is unit-level injection only.
