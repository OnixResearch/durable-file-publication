# Durable File Publication

`durable-file-publication` publishes one regular file beneath a caller-opened parent directory. The operation is bounded and capability-relative.

The crate has two parts:

- a deterministic `no_std` plus `alloc` classifier
- an optional `std` filesystem shell

On Linux and Android, the shell uses `rustix` primitives. These primitives provide exclusive no-follow staging, exact rename behavior, file synchronization, and parent synchronization.

## Start here

Run all commands from the repository root:

```console
nix develop -c cargo test --all-targets
nix develop -c cargo clippy --all-targets -- -D warnings
nix develop -c cargo check --no-default-features --target wasm32-unknown-unknown
```

## Owned operation

<!-- r[impl durable_publication.scope.one_file] -->

Each request supplies these facts:

- one caller-admitted destination leaf
- exact payload bytes and an explicit maximum
- a finite limit for stage-name collisions
- exact final Unix permission bits
- replace or race-free no-replace behavior
- visibility-only or required-durability completion

The caller opens and authorizes the parent directory before it calls the shell. The caller also supplies bounded stage names.

## Outcomes

<!-- r[impl durable_publication.outcome.commit_state] -->

- `NotCommitted` records the primary failure and the cleanup result.
- `DestinationExists` records the cleanup state without interpreting existing bytes.
- `CommittedVisibilityOnly` records rename completion without a durability claim.
- `CommittedAndParentSynchronized` records rename completion and successful parent synchronization.
- `CommittedDurabilityUnknown` records a visible destination after parent synchronization fails.

`CommittedDurabilityUnknown` is not an ordinary failure. It is also not durable success.

<!-- r[impl durable_file_publication.durability.decision] -->
<!-- r[impl durable_file_publication.durability.ack] -->

`decide_durability_acknowledgment` returns an affirmative `CommittedDurable` acknowledgment only when the publication reached parent synchronization and the caller supplies a reached intermediate-state fact. Missing, unconfirmed, visibility-only, and failed-synchronization inputs remain `CommittedDurabilityUnknown`.

The shell exposes the same decision through `durability_acknowledgment`. It does not observe replication or other external durability state.

The crate does not try to roll back after a successful rename.

## Replacement behavior

<!-- r[impl durable_publication.shell.replacement] -->

Replace mode uses one same-parent rename. No-replace mode uses `RENAME_NOREPLACE` on Linux and Android.

The crate never inspects the destination and then uses replacing rename. Preflight fails when the platform does not support the required primitive.

## Claim boundary

<!-- r[impl durable_publication.boundary.non_authority] -->

The crate does not own these functions:

- root opening or path authorization
- payload meaning or canonical encoding
- content identity or existing-content equivalence
- locks or multi-file transactions
- receipts, rollback, or recovery
- retention, deployment, garbage collection, or deletion
- release decisions
- storage-medium, replication, retention, recovery, deployment, or deletion authority

Successful synchronization calls do not prove these properties:

- filesystem correctness
- storage-device persistence
- survival after power loss
- host integrity
- payload truth

Read these documents:

- [API contract](docs/API.md)
- [Platform support](docs/PLATFORMS.md)
- [Recovery and rollback](docs/RECOVERY.md)
- [Radicle distribution](docs/DISTRIBUTION.md)
- [Source provenance](provenance/SOURCE.md)

## Distribution

The package uses immutable Radicle Git revisions. It sets `publish = false`.

Its RID is:

```text
rad:z3tAR4For7qw8ZirkJzoDw1VNDDLM
```

Consumers must pin one reviewed revision. They must use the same source identity in Cargo and Nix.

## License

All first-party packages and new owned source, contracts, schemas, fixtures, templates, documentation and policy use `AGPL-3.0-or-later`; see [`LICENSE`](LICENSE) and [`LICENSE-AGPL`](LICENSE-AGPL). Historical MIT/Apache grants remain effective for previously granted material, including byte-identical content. The retained permissive texts do not offer those terms for new contributions. Immutable provenance and third-party terms are unchanged; see [`provenance/SOURCE.md`](provenance/SOURCE.md). This source preparation does not publish a revision or migrate consumer pins.

## References

- [OnixResearch/onix-artifact](https://github.com/OnixResearch/onix-artifact) — incubation source and extraction decision at revision `c932138d880ddf4c2967f4c024b489b5c0022bf1`.
- [Mantle](https://github.com/OnixResearch/mantle) — immutable-object pilot source. Mantle retains content identity and existing-byte policy.
- [OnixOS](https://github.com/OnixResearch/onix-modules) — capability-rooted pilot source. OnixOS retains path and root authority.
