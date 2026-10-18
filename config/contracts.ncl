let Equal = fun expected =>
  std.contract.from_predicate (fun value => value == expected)
in
let NonEmptyStrings = std.contract.from_predicate (fun value =>
  std.is_array value
  && std.array.length value > 0
  && std.array.all std.is_string value
) in
let Hash = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.length value == 64
) in
let RepositoryContract = {
  schema | Equal "durable-file-publication.repository.v1",
  name | Equal "durable-file-publication",
  repository | Equal "rad://z3tAR4For7qw8ZirkJzoDw1VNDDLM",
  radicle_rid | Equal "rad:z3tAR4For7qw8ZirkJzoDw1VNDDLM",
  license | Equal "AGPL-3.0-or-later",
  packages | Equal ["durable-file-publication"],
  mechanisms | Equal ["one-file-publication"],
  features | Equal ["no_std-core", "std-shell"],
  built_in_platforms | Equal ["android", "linux"],
  runtime_dependencies | Equal ["rustix"],
  source_revision | String,
  source_manifest_blake3 | Hash,
  release_checks | NonEmptyStrings,
  non_claims | NonEmptyStrings,
} in
let ReleaseContract = {
  schema | Equal "durable-file-publication.release-profile.v1",
  project | Equal "durable-file-publication",
  repository | Equal "rad://z3tAR4For7qw8ZirkJzoDw1VNDDLM",
  radicle_rid | Equal "rad:z3tAR4For7qw8ZirkJzoDw1VNDDLM",
  license | Equal "AGPL-3.0-or-later",
  packages | Equal ["durable-file-publication"],
  required_checks | NonEmptyStrings,
  distribution | {
    channel | Equal "radicle-immutable-git-revision",
    crates_io | Equal false,
    sibling_paths | Equal false,
    floating_revisions | Equal false,
  },
  platform_evidence | {
    linux | Equal "executed",
    android | Equal "not-executed",
    other | Equal "built-in-shell-unsupported",
  },
  claim_boundary | NonEmptyStrings,
} in
{
  Repository = RepositoryContract,
  Release = ReleaseContract,
}
