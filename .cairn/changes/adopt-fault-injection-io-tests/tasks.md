## Fixture adoption

- [ ] [serial] Add `fault-injection` as a pinned dev-dependency of the shell test target and record it in the dependency catalog as transport `crates.io`, plane `validation`. r[durable_file_publication.io_fault.fixtures]
- [ ] [serial] Wrap the publication shell's staging, rename, file-sync, and parent-sync call sites with `fallible!` in test builds. r[durable_file_publication.io_fault.fixtures]
- [ ] [serial] Add a trigger function that records the injection site for fixture assertions. r[durable_file_publication.io_fault.fixtures]

## Determinism

- [ ] [serial] Add a setup guard that stores an explicit counter value and restores `u64::MAX` after each fixture. r[durable_file_publication.io_fault.determinism]
- [ ] [serial] Keep `SLEEPINESS` at zero in every fixture and in CI configuration. r[durable_file_publication.io_fault.determinism]

## Boundary

- [ ] [serial] Document that the dependency is test-only, that no runtime edge may import it, and that unit injection does not equal a ChaosControl filesystem campaign. r[durable_file_publication.io_fault.boundary]

## Verification

- [ ] [parallel] Add a positive fixture proving unchanged publication when the counter stays at `u64::MAX`. r[durable_file_publication.io_fault.verification]
- [ ] [parallel] Add negative fixtures firing at each wrapped site and asserting the declared bounded failure or `CommittedDurabilityUnknown`. r[durable_file_publication.io_fault.verification]
- [ ] [parallel] Add boundary fixtures asserting the crate, file, and line annotation and serialized counter ownership. r[durable_file_publication.io_fault.verification]
- [ ] [serial] Run package, workspace, Clippy, Cairn, and Nix checks; append the komora-io reference under `## References` in `README.md`; document non-claims. r[durable_file_publication.io_fault.verification]
