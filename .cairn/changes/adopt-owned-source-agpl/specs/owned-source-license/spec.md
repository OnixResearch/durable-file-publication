# Owned source licensing

## ADDED Requirements

### Requirement: Prospective owned-source license

r[owned_source.license.prospective] All new first-party packages and owned source, contracts, schemas, fixtures, templates, documentation and policy MUST be offered under AGPL-3.0-or-later. Current Cargo, Nickel repository/release metadata and Nix package licensing MUST agree. Historical source grants, immutable identities and third-party licenses MUST remain intact; prior grants for byte-identical material MUST NOT be represented as revoked.

#### Scenario: Current owned package is admitted

- GIVEN current owned source under AGPL-3.0-or-later
- WHEN repository and package license checks run
- THEN the current grant MUST be accepted and generated projections MUST match their Nickel sources.

#### Scenario: Wrong grant is rejected

- GIVEN current metadata offers MIT OR Apache-2.0 instead of AGPL-3.0-or-later
- WHEN the license contract runs
- THEN it MUST reject that current grant without rewriting historical provenance.
