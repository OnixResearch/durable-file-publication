let Contracts = import "../config/contracts.ncl" in
let Repository = import "../config/repository.ncl" in
# r[impl durable_publication.release.evidence]
{
  schema = "durable-file-publication.release-profile.v1",
  project = Repository.name,
  repository = Repository.repository,
  radicle_rid = Repository.radicle_rid,
  license = Repository.license,
  packages = Repository.packages,
  required_checks = Repository.release_checks,
  distribution = {
    channel = "radicle-immutable-git-revision",
    crates_io = false,
    sibling_paths = false,
    floating_revisions = false,
  },
  platform_evidence = {
    linux = "executed",
    android = "not-executed",
    other = "built-in-shell-unsupported",
  },
  claim_boundary = Repository.non_claims,
} | Contracts.Release
