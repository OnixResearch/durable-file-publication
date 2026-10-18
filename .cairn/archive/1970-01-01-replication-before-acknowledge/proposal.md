## Why

Durable File Publication owns the bounded publication of one regular file beneath a caller-opened parent. It already reports `CommittedDurabilityUnknown` when parent synchronization fails. It does not yet enforce an explicit durability-acknowledgment contract: an acknowledgment that the publication is durable is returned only after a durable intermediate state is reached.

Celld, the self-hosted Durable Objects daemon, does not acknowledge a replicated write before the data is durable in object storage (RPO=0). A node loss therefore cannot lose an acknowledged write. That discipline transfers directly to file publication: never acknowledge durability you have not reached.

Durable File Publication should own the durability-acknowledgment contract. The caller owns the medium, retention, recovery, and deletion authority.

## What Changes

- Add an explicit durability-acknowledgment contract to the publication decision.
- Report an affirmative durable acknowledgment only after an intermediate durability state is reached.
- Report `CommittedDurabilityUnknown` whenever durability cannot be confirmed; never fabricate an affirmative ack.
- Keep request validation and decision classification in the pure core.
- Add positive, negative, and boundary fixtures for ack, unknown, race, and sync-failure paths.

## Impact

The change gives callers a truthful durability signal. It does not prove retention, deploy it to a store, or define the durable medium.

Publication identities and receipts remain BLAKE3.

## Dependencies

This change has no downstream prerequisite. Artifact and other consumers consume the reviewed ack contract after this change stabilizes.

## Non-goals

- Do not add a new durable medium or transport.
- Do not implement replication, hashing of external stores, or object storage.
- Do not weaken the existing `CommittedDurabilityUnknown` result.
- Do not claim parity with, or equivalence to, the Celld durable-object storage model.
