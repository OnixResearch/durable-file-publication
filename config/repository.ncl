let Contracts = import "contracts.ncl" in
# r[impl durable_publication.repository.identity]
# r[impl durable_publication.boundary.non_authority]
{
  schema = "durable-file-publication.repository.v1",
  name = "durable-file-publication",
  repository = "rad://z3tAR4For7qw8ZirkJzoDw1VNDDLM",
  radicle_rid = "rad:z3tAR4For7qw8ZirkJzoDw1VNDDLM",
  license = "AGPL-3.0-or-later",
  packages = ["durable-file-publication"],
  mechanisms = ["one-file-publication"],
  features = ["no_std-core", "std-shell"],
  built_in_platforms = ["android", "linux"],
  runtime_dependencies = ["rustix"],
  source_revision = "c932138d880ddf4c2967f4c024b489b5c0022bf1",
  source_manifest_blake3 = "bcb84e58099fbc11cc2d0e6c3ce8e3f0013cfeb3618094379891f4d9d3eee267",
  release_checks = [
    "cargo-format",
    "cargo-test",
    "cargo-clippy",
    "core-no-std-host",
    "core-no-std-wasm",
    "nickel-contracts",
    "source-provenance",
    "cairn-validation",
    "authority-audit",
  ],
  non_claims = [
    "Publication does not prove destination authorization, payload truth, canonical encoding, or content identity.",
    "Successful synchronization calls do not prove filesystem correctness, storage-device persistence, or power-loss survival.",
    "CommittedDurabilityUnknown is a committed destination observation and is not ordinary failure or durable success.",
    "The crate does not own existing-content equivalence, locks, transactions, receipts, rollback, recovery, retention, deployment, garbage collection, deletion, or release decisions.",
    "Linux execution evidence does not prove Android execution or support on other operating systems.",
  ],
} | Contracts.Repository
