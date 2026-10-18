## Goals

- Make every declared I/O failure path of the publication shell triggerable from a test.
- Keep injection deterministic, annotated, and confined to test builds.
- Change no publication semantics.

## Mechanism

The `fault-injection` crate exposes the `fallible!` expression macro. Each wrapped call site decrements the process-global `FAULT_INJECT_COUNTER` atomic. When the counter reaches zero the macro returns an `io::Error` whose description carries the crate, file, and line of the injection site. The counter defaults to `u64::MAX`, so unwrapped behavior is unchanged until a test stores an explicit value.

The shell wraps only its own I/O calls: staging writes, the final rename, file synchronization, and parent-directory synchronization. The pure core is not touched; it already classifies supplied observations.

## Determinism controls

- Each fixture stores an explicit counter value before acting and asserts one classified outcome.
- Fixtures that share the process-global counter run serially in one test thread, or reset the counter in a setup guard.
- `SLEEPINESS` stays zero in every test and in CI. Random delays are out of scope.
- The trigger function records the injection site so a fixture can assert which site fired.

## Placement

`fault-injection` is a dev-dependency of the shell test target only. No public API, feature flag, or runtime code path learns about it. The workspace dependency catalog records transport `crates.io` and plane `validation`.

## Verification

Positive coverage: a counter left at `u64::MAX` completes a publication unchanged.

Negative coverage: counter values that fire at the staging write, the rename, the file sync, and the parent sync, each asserting the declared bounded failure or `CommittedDurabilityUnknown`.

Boundary coverage: an injected error carries the expected crate, file, and line annotation; parallel fixtures never observe a foreign counter value; no fixture enables `SLEEPINESS`.
