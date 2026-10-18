# Platform support

<!-- r[impl durable_publication.platform.support] -->

| Platform | Replace | No-replace | Parent sync | Evidence |
|---|---|---|---|---|
| Linux | `renameat` | `renameat_with(NOREPLACE)` | Directory `fsync` | Executed on `x86_64-linux` |
| Android | Same compiled branch | Same compiled branch | Directory `fsync` | Not executed in this repository validation |
| Other targets | No built-in shell | Unsupported | Unsupported | Pure core only where `alloc` exists |

Linux integration checks use a caller-opened directory file. They verify payload bytes, exact permission bits, symlink stage-name collision handling, destination preservation under no-replace, replacing rename, and directory synchronization completion.

Android shares the guarded implementation branch. This fact is compile-structure evidence, not physical Android execution evidence.

The `no_std` core builds for `wasm32-unknown-unknown`. This does not make the filesystem shell available on WebAssembly.
