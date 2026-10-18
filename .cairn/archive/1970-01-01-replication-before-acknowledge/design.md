## Goals

- Report durability honestly.
- Return an affirmative durable acknowledgment only after a durability intermediate state is reached.
- Return `CommittedDurabilityUnknown` whenever durability cannot be confirmed.
- Keep decision classification in the pure core and filesystem effects in the shell.

## Decision classification

The pure core classifies a publication into one of three verdicts:

1. `CommittedDurable` — the caller-declared intermediate durability state was reached.
2. `CommittedDurabilityUnknown` — the file committed but durability cannot be confirmed.
3. A bounded failure — the publication did not commit.

The core accepts the declared durability observation as a value. It does not read files, clocks, or external state.

## Acknowledgment contract

The shell exposes one acknowledgment surface.

The shell returns an affirmative durable acknowledgment only when the caller-declared intermediate durability state was reached. Otherwise it returns `CommittedDurabilityUnknown` or a failure. It never returns an affirmative ack for an unconfirmed state.

The acknowledgment names the exact decision. It does not claim retention, recovery, deployment, or deletion authority.

## Functional core and shell

The core decides and classifies. It accepts already-opened and authorized parent capabilities and the declared durability observation.

The shell performs the bounded filesystem effect, including the rename and any parent synchronization, then reports the disposition.

The caller owns the medium, retention, recovery, and deletion policy.

## Reference

The Celld no-acknowledge-before-durable discipline is a bounded reference. It is a comparison source, not a Durable File Publication requirement, parity claim, or equivalence claim.

## Verification

Positive coverage includes a reached intermediate durability state producing an affirmative ack.

Negative coverage includes a failed rename, a failed synchronization, and a race that prevents durability confirmation.

Boundary coverage rejects an affirmative ack for an unconfirmed state, a missing durability fact, an ambient root open, and any retention or recovery claim.
