# Recovery and rollback

## Before rename

The shell attempts to remove its private stage after write, permission, payload-sync, rename, or destination-exists termination.

`CleanupDisposition::Failed` means the consumer must inspect the known parent under its own stale-stage and authorization policy. The crate does not scan or delete unknown stage names.

## After rename

<!-- r[impl durable_publication.shell.durability] -->

Rename makes the destination visible. If parent synchronization then fails, the result is `CommittedDurabilityUnknown`.

The consumer must:

1. Treat the destination as committed and possibly visible.
2. Inspect current product state through its authorized capability.
3. Apply product-owned recovery and evidence rules.
4. Avoid an automatic old-destination assumption.

The crate does not roll back after rename. A second rename would create another commit and could hide the first observation.

## Consumer adoption rollback

A consumer must keep its prior immutable implementation until its adoption checks pass. Adoption rollback restores the old dependency revision and adapter.

This repository does not authorize product rollback, retention, garbage collection, or deletion.
