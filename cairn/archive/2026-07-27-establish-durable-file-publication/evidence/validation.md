# Validation evidence

<!-- r[verify durable_publication.release.evidence] -->

Date: 2026-07-27

## Result boundary

This evidence validates one independent crate, its deterministic core, its Linux filesystem shell, its typed contracts, and its lifecycle linkage. It does not prove permanent Mantle or OnixOS adoption.

The repository started from the reviewed Artifact extraction decision at `c932138d880ddf4c2967f4c024b489b5c0022bf1`. The approved disposable candidate revision was `67f14770a82e24eb39d3a751285b908232566e6b`.

## Baseline

Task `962` passed the original Artifact candidate tests, strict Clippy, and `wasm32-unknown-unknown` core check before extraction.

The original shared corpus BLAKE3 was `d02379677341b7408b636906c4bee4910ef3835cc81261373194ef84678b27da`.

## Approach and audit

Four mechanisms were reviewed:

- One separate feature-gated crate: selected.
- Separate core and shell crates: deferred because the second release boundary had no independent need.
- An Artifact workspace package: rejected because Artifact owns pure reusable cores.
- A copied consumer helper: rejected because it would import product policy and licensing risk.

The advisory review agreed on a separate repository. Its suggestion to synchronize the parent before rename was rejected. Parent synchronization remains after rename because it covers the destination entry.

The adversarial audit found that an `O_PATH` directory could reach rename before parent-sync capability was known. Commit `dc362006b17f6816a82ca89e4bc455b7a03c8fcb` added a synchronizability probe and a negative test. Durability requests now reject that handle before stage creation. A later parent-sync failure still returns `CommittedDurabilityUnknown`.

## Source identities

Reviewed source identities:

- `core.rs`: `ae4a3ee7940b6b782082e9370587f219913e545a3cdbfaeaa9d7eca856b95fec`
- `corpus.rs`: `d02379677341b7408b636906c4bee4910ef3835cc81261373194ef84678b27da`
- `shell.rs`: `a9a4fcf75f555d2b2dd9bdac0f2363a359c86d1b496cd556d5dd1edd2ed97ea7`
- `pilot.rs`: `4e8fbd66b02e0d3306aa9483e383c579a4287e87a9537e49e32c62d6ed576193`

Promoted source identities:

- `src/lib.rs`: `4bcb234a0d2ffb1c202a19ae41ef5a6b9529124abb9bbe002c2263049391d129`
- `src/core.rs`: `4d040eb172f0fc518f5fa662c1ffa0e4457319921e39fb7a6c1f76d994665469`
- `src/corpus.rs`: `d02379677341b7408b636906c4bee4910ef3835cc81261373194ef84678b27da`
- `src/shell.rs`: `3fcf209f905c47aac62c30d97e7ae0dfb276949d2eebe68b1298c76d5b0bc05e`
- Source-provenance JSON: `bcb84e58099fbc11cc2d0e6c3ce8e3f0013cfeb3618094379891f4d9d3eee267`

The Nix provenance check fetches the immutable Artifact revision, verifies all reviewed hashes, verifies promoted hashes, and binds the source-manifest hash to the repository projection.

## Cargo and platform checks

Task `1032` passed after the adversarial fix:

- Formatting.
- Six conformance tests over the 17-case shared corpus and additional structural cases.
- Three Linux capability integration tests.
- Strict Clippy for all targets.
- `no_std` core check for `wasm32-unknown-unknown`.

The Linux tests exercise exclusive no-follow staging, symlink collisions, exact permissions, replace, `RENAME_NOREPLACE`, destination preservation, payload synchronization, parent synchronization, invalid parent handles, and `O_PATH` durability rejection before mutation.

Android has a guarded built-in branch but was not executed. Other operating systems have no built-in shell. The WebAssembly check proves only that the pure core builds there.

## Lifecycle and repository checks

Proposal, design, tasks, and repository validation gates passed.

Accepted specification synchronization receipt: `2629c7b464ae7ec4d85edb17d05af43b215f71e711caa850823c6bdac47aa5e7`.

Focused traceability covers 12 of 12 requirements. The Radicle-replication receipt is `dd4548d6244442c6cafb25b2496696fada02a8f854ba7e570588529d6c725a58`.

Tasks `1052` and `1062` passed `nix flake check -L` on `x86_64-linux`. The final pre-archive run included completed evidence, provenance, projections, and task state. Post-archive tasks `1071` and `1104` also passed, including Radicle-only repository metadata. Nix reported `all checks passed` and omitted incompatible `aarch64-linux` outputs on this host.

## Radicle publication

No GitHub repository or remote is used. The repository was published through Radicle as `rad:z3tAR4For7qw8ZirkJzoDw1VNDDLM` by delegate `did:key:z6MksnXbFoE8zkCkGWhHc8zuxpnEUhrJHv2KECRV4GSv9gkx`.

Initial publication pushed revision `732b9c9b8b41888849056619def8b7692bc9e6f7`, announced the repository to the network, and set `rad` as the `main` branch upstream. Later metadata commits retain the same RID and use only that remote.

A later replication pass established two checked paths. Node `paintedlife` (`z6Mkkyj4NSrjzR7VEnzbG1Km9f66qTn3d7SApqEH9rXAVR7A`) reported the publisher signed refs in sync. Aspen1 uses the separate seed identity `did:key:z6Mkp4REgQ3K6EWisqRCcUv1pBsgTp3WUGcEAmVHbBYy2bkL`. Its private-SSH bootstrap used checksum comparison, passed `git fsck --full --strict`, resolved both `main` references to `39848ec483f351b71d3dd98f0886504d54ba7eb5`, and produced a clean Radicle checkout. Aspen1 then published its matching fork through Radicle; `paintedlife` and `garden.bastidehub.xyz` reported those signed refs in sync.

Direct Radicle fetch handshakes to Aspen1 reset during bootstrap, so the evidence does not claim a Radicle-only bootstrap path. The public repository remains Radicle-only. The replica checks do not prove indefinite availability, reboot persistence, seed trust, or future synchronization.

## Non-claims

These checks do not prove filesystem correctness, storage-device persistence, power-loss survival, host integrity, payload truth, destination authorization, consumer integration, deployment success, Android execution, or release eligibility in a consumer repository.

Archive execution passed with receipt `3a11ed34c922a32678f4e5e72bd9ea48b3e3d0eba35edf0c821c27f5a7920fe4`.
