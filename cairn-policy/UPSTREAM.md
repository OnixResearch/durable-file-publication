# Cairn policy provenance

The local lifecycle policy is vendored from the reviewed Cairn source revision `fde71b298b61856936c35b701876d4933393f0df`.

The files in this directory are copies of `cairn-policy/*.ncl` from that revision. The local policy carries no field-level edits.

The generated file `generated/cairn-policy.json` is the runtime input. Refresh it with the reviewed Cairn provider:

```sh
cairn policy export --root . --output cairn-policy/generated/cairn-policy.json
```

Then run policy freshness, lifecycle validation, lifecycle gates, and traceability checks.
