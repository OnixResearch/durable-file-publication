# Agent Notes

## Purpose

This repository owns one narrow mechanical operation: bounded publication of one regular file beneath one caller-opened parent directory.

## Boundaries

- Keep request validation and observation classification in the pure `no_std + alloc` core.
- Keep filesystem effects in the optional `std` shell.
- Accept already-opened and authorized parent capabilities. Never open ambient roots.
- Require explicit payload, collision, permission, replacement, and durability facts.
- Preserve `CommittedDurabilityUnknown` after rename succeeds and parent synchronization fails.
- Never implement no-replace as inspection followed by replacing rename.
- Keep product identity, equality, receipts, authority, recovery, retention, deployment, and deletion outside the crate.

## Tests

Every behavior change needs positive and negative tests. Include malformed leaves, bounds, unsupported primitives, collisions, operation failures, cleanup failures, destination races, partial commits, and platform limits.

## Checks

```console
nix develop -c cargo fmt --all -- --check
nix develop -c cargo test --all-targets
nix develop -c cargo clippy --all-targets -- -D warnings
nix develop -c cargo check --no-default-features --target wasm32-unknown-unknown
nickel typecheck config/repository.ncl
nickel typecheck release/profile.ncl
nix run path:../cairn#cairn -- validate --root . --policy cairn-policy/generated/cairn-policy.json
nix flake check -L
```

Use external `TMPDIR` and `CARGO_TARGET_DIR` for broad checks. Do not edit `Cargo.lock` or `flake.lock` manually.
