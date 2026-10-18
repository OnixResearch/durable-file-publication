# Radicle distribution

The repository is published only through Radicle.

- RID: `rad:z3tAR4For7qw8ZirkJzoDw1VNDDLM`
- Delegate: `did:key:z6MksnXbFoE8zkCkGWhHc8zuxpnEUhrJHv2KECRV4GSv9gkx`
- Default branch: `main`
- Git remote: `rad://z3tAR4For7qw8ZirkJzoDw1VNDDLM`

Clone with:

```console
rad clone rad:z3tAR4For7qw8ZirkJzoDw1VNDDLM
```

Consumers must pin an exact Git revision from this RID. A Radicle RID identifies the repository. It does not replace the immutable source revision required by Cargo, Nix, provenance, and release checks.

## Replication

Britton-desktop owns the delegate identity. Aspen1 uses the separate seed identity `did:key:z6Mkp4REgQ3K6EWisqRCcUv1pBsgTp3WUGcEAmVHbBYy2bkL`. Validation checked Aspen1 storage, Git object integrity, canonical references, and a clean Radicle checkout.

The Aspen1 bootstrap used private SSH after direct Radicle fetch handshakes reset. Public distribution remains Radicle-only.

Announcement and synchronization show network availability only. They do not prove source correctness, seed trust, reboot persistence, indefinite replication, consumer adoption, or release eligibility.
