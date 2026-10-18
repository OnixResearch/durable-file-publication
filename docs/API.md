# API contract

## Functional core

<!-- r[impl durable_publication.core.classification] -->

`core::validate_request` checks one request without side effects. `core::classify_publication` checks one finite ordered observation stream.

The core accepts these success phases in order:

1. Create a private stage.
2. Write the exact payload byte count.
3. Apply exact final permissions.
4. Synchronize the staged payload.
5. Rename the stage to the destination.
6. Synchronize the parent when durability is required.

The classifier rejects impossible phase order, incorrect byte counts, unsafe stage leaves, missing cleanup, and incomplete terminal observations.

The core does not prove that supplied observations are true.

## Durability acknowledgment

<!-- r[impl durable_file_publication.durability.decision] -->
<!-- r[impl durable_file_publication.durability.ack] -->
<!-- r[impl durable_file_publication.durability.boundary] -->

`core::decide_durability_acknowledgment` accepts one publication disposition and one caller-supplied intermediate durability observation.

It returns `CommittedDurable` only when both these facts hold:

1. Parent synchronization completed.
2. The caller declares that its required intermediate durability state was reached.

Every committed but absent, unconfirmed, visibility-only, or failed-synchronization case returns `CommittedDurabilityUnknown`. Pre-commit failures remain `PublicationNotCommitted` and retain the exact publication disposition.

`shell::durability_acknowledgment` delegates to this pure decision. Neither API observes replication, storage persistence, retention, recovery, deployment, or deletion authority.

## Imperative shell

<!-- r[impl durable_publication.shell.preflight] -->
<!-- r[impl durable_publication.shell.staging] -->

`shell::publish_one_file` accepts:

- One `PublicationIo` over a caller-opened parent.
- One injected `StageNameSource`.
- One validated mechanical `PublicationRequest`.
- One payload byte slice.

The shell completes request admission and primitive preflight before it requests a stage name. It then records exact observations and asks the pure core to classify them.

`PublicationIo` is the capability boundary. Implementations must keep every operation relative to the same opened parent.

## Linux and Android adapter

`LinuxDirectory::from_open_directory` accepts a caller-opened directory file. It probes parent synchronization once and records unsupported capability handles, such as `O_PATH`, for preflight. The adapter uses `openat`, `renameat`, `renameat_with`, `unlinkat`, file synchronization, and directory `fsync`.

The caller must open and authorize the directory. This crate does not recover an ambient path or choose a product root.

## Failure handling

A failure before rename triggers one stage cleanup attempt when a stage exists. The result retains cleanup success or failure.

A failure after rename never triggers implicit rollback. Parent-sync failure returns `CommittedDurabilityUnknown`.
