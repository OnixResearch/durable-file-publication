# Establish durable file publication

## Goal

Create an independent product-neutral repository for bounded publication of one regular file beneath one caller-opened parent directory.

## Why

Artifact commit `c932138d880ddf4c2967f4c024b489b5c0022bf1` completed two consumer-shaped pilots and approved extraction into a separate repository. The disposable fixture has no independent release identity, lifecycle, immutable revision, or permanent consumer pin.

This change promotes the reviewed mechanism without moving product authority into the shared crate. It preserves race-free no-replace, explicit durability, and the `CommittedDurabilityUnknown` partial-commit result.

## Scope

- Publish one newly staged regular file to one admitted leaf.
- Keep a deterministic `no_std + alloc` core and an optional `std` filesystem shell.
- Support the Linux and Android `rustix` primitive branch.
- Export one shared positive and negative conformance corpus.
- Record exact Artifact provenance and a BLAKE3 source identity.
- Add typed Nickel repository and release contracts.
- Add Cargo, Nix, Cairn, and traceability release gates.

## Out of scope

The repository does not own root opening, path authorization, payload meaning, canonical encoding, existing-content equivalence, locks, multi-file transactions, receipts, rollback, recovery, retention, deployment, garbage collection, or deletion authority.

Permanent Mantle and OnixOS adoption remains separate consumer-owned work pinned to an immutable repository revision.
