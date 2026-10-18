## Context

The current first-party package grant is MIT OR Apache-2.0. Operator authorization selects AGPL-3.0-or-later for all owned material. Historical grants and pinned source identities remain factual records.

## Decisions

### Decision: Apply a prospective owned-source grant

**Choice:** Use AGPL-3.0-or-later in authoritative Cargo, repository/release contracts, owned fixtures and Nix guard expectations. Ship the full AGPL text and clarify the retained permissive texts. Regenerate JSON through the existing Nickel exports.

**Rationale:** One current owned-source policy avoids contradictory permissive offers while preserving historical rights. Repository-level scope includes owned source, documentation, policy, schemas, templates and fixtures without modifying immutable baselines or third-party code.

### Decision: Preserve prior grants and source identities

**Choice:** Keep historical provenance licenses, hashes, receipts, source manifests and all consumer pins intact. Add prospective clarification rather than rewriting history; byte-identical previously granted content remains available under its earlier terms.

**Rationale:** Prior grants cannot be revoked. A changed metadata field cannot retroactively change an immutable revision or third-party license.

## Risks / Trade-offs

- AGPL obligations differ from permissive grants; receiver-local adoption remains separate.
- Existing immutable receipts may still mention old terms correctly; they are not current offers for new contributions.
- Stage gates establish structural admission only. Final verification, publication and release readiness are not claimed.
