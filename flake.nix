{
  description = "Bounded capability-relative publication of one regular file";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    cairn = {
      url = "github:OnixResearch/cairn/cf5c4a5c10361ca20a13addf14954407d6857028";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    artifact-source = {
      url = "github:OnixResearch/onix-artifact/c932138d880ddf4c2967f4c024b489b5c0022bf1";
      flake = false;
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      cairn,
      artifact-source,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      eachSystem = function: nixpkgs.lib.genAttrs systems (system: function system);
    in
    {
      packages = eachSystem (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          rustPlatform = pkgs.makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          };
        in
        {
          durable-file-publication = rustPlatform.buildRustPackage {
            pname = "durable-file-publication";
            version = "0.1.0";
            src = self;
            cargoLock.lockFile = ./Cargo.lock;
            doCheck = true;
            installPhase = ''
              mkdir -p "$out/share/doc/durable-file-publication" "$out/share/licenses/durable-file-publication"
              cp README.md "$out/share/doc/durable-file-publication/"
              cp -R docs provenance "$out/share/doc/durable-file-publication/"
              cp LICENSE LICENSE-AGPL LICENSE-MIT LICENSE-APACHE NOTICE "$out/share/licenses/durable-file-publication/"
            '';
            meta.license = pkgs.lib.licenses.agpl3Plus;
          };
          default = self.packages.${system}.durable-file-publication;
        }
      );

      checks = eachSystem (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          rustPlatform = pkgs.makeRustPlatform {
            cargo = toolchain;
            rustc = toolchain;
          };
          noStdCheck =
            name: target:
            rustPlatform.buildRustPackage {
              pname = name;
              version = "0.1.0";
              src = self;
              cargoLock.lockFile = ./Cargo.lock;
              cargoBuildFlags = [ "--no-default-features" ]
              ++ pkgs.lib.optionals (target != null) [
                "--target"
                target
              ];
              doCheck = false;
              installPhase = ''
                touch "$out"
              '';
            };
        in
        {
          cargo-test = rustPlatform.buildRustPackage {
            pname = "durable-file-publication-tests";
            version = "0.1.0";
            src = self;
            cargoLock.lockFile = ./Cargo.lock;
          };

          cargo-clippy = rustPlatform.buildRustPackage {
            pname = "durable-file-publication-clippy";
            version = "0.1.0";
            src = self;
            cargoLock.lockFile = ./Cargo.lock;
            buildPhase = ''
              cargo clippy --all-targets --offline -- --deny warnings
            '';
            doCheck = false;
            installPhase = ''
              touch "$out"
            '';
          };

          cargo-format = pkgs.runCommand "durable-file-publication-format" {
            nativeBuildInputs = [ toolchain ];
            src = self;
          } ''
            set -eu
            cd "$src"
            cargo fmt --all -- --check
            touch "$out"
          '';

          core-no-std-host = noStdCheck "durable-file-publication-no-std-host" null;
          core-no-std-wasm = noStdCheck "durable-file-publication-no-std-wasm" "wasm32-unknown-unknown";

          # r[verify durable_publication.repository.identity]
          source-provenance = pkgs.runCommand "durable-file-publication-source-provenance" {
            nativeBuildInputs = [
              pkgs.b3sum
              pkgs.jq
              pkgs.nickel
            ];
            src = self;
          } ''
            set -eu
            cd "$src"
            nickel typecheck provenance/source.ncl
            nickel export --format json provenance/source.ncl > "$TMPDIR/source.json"
            cmp "$TMPDIR/source.json" provenance/source.json
            actual_manifest_hash="$(b3sum provenance/source.json | cut -d ' ' -f 1)"
            expected_manifest_hash="$(jq -r '.source_manifest_blake3' config/repository.json)"
            test "$actual_manifest_hash" = "$expected_manifest_hash"
            test "$(b3sum "${artifact-source}/fixtures/publication-candidate/src/core.rs" | cut -d ' ' -f 1)" = "ae4a3ee7940b6b782082e9370587f219913e545a3cdbfaeaa9d7eca856b95fec"
            test "$(b3sum "${artifact-source}/fixtures/publication-candidate/src/corpus.rs" | cut -d ' ' -f 1)" = "d02379677341b7408b636906c4bee4910ef3835cc81261373194ef84678b27da"
            test "$(b3sum "${artifact-source}/fixtures/publication-candidate/src/shell.rs" | cut -d ' ' -f 1)" = "a9a4fcf75f555d2b2dd9bdac0f2363a359c86d1b496cd556d5dd1edd2ed97ea7"
            test "$(b3sum "${artifact-source}/fixtures/publication-candidate/src/pilot.rs" | cut -d ' ' -f 1)" = "4e8fbd66b02e0d3306aa9483e383c579a4287e87a9537e49e32c62d6ed576193"
            test "$(b3sum src/lib.rs | cut -d ' ' -f 1)" = "4bcb234a0d2ffb1c202a19ae41ef5a6b9529124abb9bbe002c2263049391d129"
            test "$(b3sum src/core.rs | cut -d ' ' -f 1)" = "2253bf7ad3d51ecc1a7c1bde7e33803880ba9b394aa70316859a84f5807733c3"
            test "$(b3sum src/corpus.rs | cut -d ' ' -f 1)" = "d02379677341b7408b636906c4bee4910ef3835cc81261373194ef84678b27da"
            test "$(b3sum src/shell.rs | cut -d ' ' -f 1)" = "b72c345e833871330bb301f9db9862369912e56500148052f4bff4cfbe9b5787"
            touch "$out"
          '';

          # r[verify durable_publication.release.evidence]
          nickel-contracts = pkgs.runCommand "durable-file-publication-nickel-contracts" {
            nativeBuildInputs = [ pkgs.nickel ];
            src = self;
          } ''
            set -eu
            cd "$src"
            nickel typecheck config/repository.ncl
            nickel typecheck release/profile.ncl
            nickel export --format json config/repository.ncl > "$TMPDIR/repository.json"
            nickel export --format json release/profile.ncl > "$TMPDIR/profile.json"
            cmp "$TMPDIR/repository.json" config/repository.json
            cmp "$TMPDIR/profile.json" release/profile.json
            touch "$out"
          '';

          cairn-validation = pkgs.runCommand "durable-file-publication-cairn-validation" {
            nativeBuildInputs = [
              cairn.packages.${system}.cairn
              pkgs.nickel
            ];
            src = self;
          } ''
            set -eu
            cp -R --no-preserve=mode "$src" work
            cd work
            nickel export --format json cairn-policy/default.ncl > "$TMPDIR/cairn-policy.json"
            cmp "$TMPDIR/cairn-policy.json" cairn-policy/generated/cairn-policy.json
            cairn validate --root . --policy cairn-policy/generated/cairn-policy.json
            cairn traceability coverage --root . --policy cairn-policy/generated/cairn-policy.json --json > "$TMPDIR/traceability.json"
            touch "$out"
          '';

          # r[verify durable_publication.boundary.non_authority]
          authority-audit = pkgs.runCommand "durable-file-publication-authority-audit" {
            nativeBuildInputs = [
              pkgs.gnugrep
              pkgs.jq
              toolchain
            ];
            src = self;
          } ''
            set -eu
            if grep -E 'std::fs|std::env|std::process|std::time|std::net|tokio::|async |println!|eprintln!' "$src/src/core.rs" "$src/src/corpus.rs"; then
              echo "pure core gained an imperative dependency" >&2
              exit 1
            fi
            if grep -R -i -E 'mantle|onixos|valence|cairn|artifact' "$src/src" "$src/Cargo.toml" --include='*.rs' --include='Cargo.toml'; then
              echo "public crate surface gained a product dependency or term" >&2
              exit 1
            fi
            export CARGO_HOME="$TMPDIR/cargo-home"
            cargo metadata --manifest-path "$src/Cargo.toml" --format-version 1 --no-deps --offline > "$TMPDIR/metadata.json"
            test "$(jq '.packages | length' "$TMPDIR/metadata.json")" -eq 1
            test "$(jq -r '.packages[0].license' "$TMPDIR/metadata.json")" = "AGPL-3.0-or-later"
            test "$(jq -r '[.packages[0].dependencies[] | select(.kind == null) | .name] | sort | join(",")' "$TMPDIR/metadata.json")" = "rustix"
            touch "$out"
          '';
        }
      );

      devShells = eachSystem (
        system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ (import rust-overlay) ];
          };
          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in
        {
          default = pkgs.mkShell {
            packages = [
              toolchain
              pkgs.b3sum
              pkgs.jq
              pkgs.nickel
            ];
          };
        }
      );
    };
}
