# Design

## Completion contract

The change is complete only when the new repository has an independently useful crate, exact source provenance, positive and negative conformance evidence, passing release checks, synchronized accepted requirements, and an archived Cairn receipt.

A copied fixture, passing happy-path test, task checkbox, unsupported no-replace fallback, hidden partial commit, sibling-path consumer dependency, or unpublished local revision is not completion.

## Approach registry

| Family | Mechanism | State | Reason |
|---|---|---|---|
| Separate feature-gated crate | One crate contains a `no_std + alloc` functional core and an optional `std` shell | Selected | This is the smallest release identity that preserves the reviewed boundary. |
| Split core and shell crates | One core crate plus one filesystem crate | Deferred | This adds version and dependency coordination without a second independent consumer need. |
| Artifact workspace package | Add a std mutation package to Artifact | Rejected | Artifact owns pure reusable cores and explicitly approved a separate repository. |
| Consumer helper extraction | Copy Mantle or OnixOS implementation | Rejected | This imports product policy, creates licensing risk, and does not establish shared ownership. |

The independent advisory review agreed on a separate repository. Its suggestion to synchronize the parent before rename was rejected because it would not establish destination-entry durability. The reviewed order remains payload synchronization, rename, then parent synchronization.

## Functional core

The core validates already-admitted request facts and classifies a finite ordered observation stream. It does not perform I/O, read clocks, inspect environment, generate names, serialize receipts, or claim that observations are true.

The core retains exact commit state. Parent-sync failure after rename yields `CommittedDurabilityUnknown`. It never becomes ordinary failure or durable success.

## Imperative shell

The shell accepts one caller-opened parent capability through `PublicationIo`, one admitted destination leaf, payload bytes, explicit bounds, exact permission bits, replacement behavior, durability behavior, and an injected stage-name source.

Admission and platform preflight finish before stage creation. The shell creates a private no-follow regular stage, writes all bytes, applies exact permissions, synchronizes the stage, performs one same-parent rename, and synchronizes the parent when required.

No-replace uses a race-free primitive or returns unsupported before mutation. Inspection followed by replacing rename is forbidden.

## Platform boundary

Linux and Android use `rustix::fs::renameat_with(NOREPLACE)` and directory `fsync`. Linux receives executed integration evidence. Android shares the compiled branch but is not claimed as executed until a target runner supplies evidence. Other operating systems expose no built-in shell and remain unsupported.

## Provenance and license

The implementation is promoted from the clean-room Artifact fixture at reviewed Artifact revision `c932138d880ddf4c2967f4c024b489b5c0022bf1`. The extraction records the fixture files and their BLAKE3 identities. Artifact grants `MIT OR Apache-2.0`; no Mantle or OnixOS implementation text is imported.

Consumer-shaped pilots stay in test evidence. Product names and policy types do not enter the public crate API.

## Adversarial audit

The validation must attempt these failures:

- malformed destination and stage leaves;
- zero or exceeded byte and collision bounds;
- invalid permission bits and payload-length mismatch;
- unsupported no-replace and parent synchronization before I/O;
- stage collisions and stage-create failure;
- write, permission, payload-sync, rename, and parent-sync failure;
- cleanup success and cleanup failure;
- destination race under no-replace;
- impossible phase order and observations after terminal state;
- non-directory parent handles and colliding symlink stage names.

## Release and rollback

The release surface includes the crate, conformance corpus, platform matrix, typed Nickel contracts, Nix checks, Cairn requirements, and source provenance.

Consumers retain their current implementations until separate adoption changes pass. A consumer rollback restores its prior immutable dependency and adapter. This repository does not perform product rollback after a committed rename.
