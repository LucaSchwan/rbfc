{
  description = "A rust brainfuck compiler";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = pkgs.rust-bin.stable.latest.default;
        rustPlatform = pkgs.makeRustPlatform {
          rustc = rustVersion;
          cargo = rustVersion;

        };
        myRustBuild = rustPlatform.buildRustPackage {
          pname = "flake_tests";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
      in {
        packages = rec {
          default = myRustBuild;
          docker = pkgs.dockerTools.buildLayeredImage {
            name = default.pname;
            tag = default.version;
            contents = [ default ];

            config = { Cmd = [ "${default}/bin/flake_tests" ]; };
          };
        };
        devShells.default = pkgs.mkShell {
          buildInputs = [
            (rustVersion.override {
              extensions = [ "rust-src" "rust-analyzer" "rustfmt" ];
            })
          ];
        };
      });
}
