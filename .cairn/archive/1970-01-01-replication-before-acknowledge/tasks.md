## Durability decision

- [x] [serial] Add the explicit durability-acknowledgment decision and disposition types to the pure core. r[durable_file_publication.durability.decision]
- [x] [serial] Classify the affirmative durable acknowledgment only from a reached intermediate durability state. r[durable_file_publication.durability.ack]
- [x] [serial] Preserve the `CommittedDurabilityUnknown` result for every unconfirmed durable state. r[durable_file_publication.durability.ack]
- [x] [serial] Document that the caller owns the medium, retention, recovery, and deletion authority. r[durable_file_publication.durability.boundary]

## Verification

- [x] [parallel] Add positive cases for a reached intermediate state producing an affirmative ack. r[durable_file_publication.durability.verification]
- [x] [parallel] Add negative cases for failed rename, failed synchronization, and durability races. r[durable_file_publication.durability.verification]
- [x] [parallel] Add boundary cases that reject an affirmative ack for an unconfirmed state and an absent durability fact. r[durable_file_publication.durability.verification]
- [x] [serial] Run no-std, Wasm, package, workspace, Clippy, Cairn, and Nix checks, then document non-claims. r[durable_file_publication.durability.verification]
