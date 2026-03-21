{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      crane,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        craneLib = (crane.mkLib pkgs).overrideToolchain (
          p:
          p.rust-bin.stable.latest.default.override {
            extensions = [
              "rust-src"
              "rust-analyzer"
              "clippy"
            ];
            targets = [
            "riscv32imac-unknown-none-elf"
            "wasm32-unknown-unknown"
            ];
          }
        );

        commonArgs = {
          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            name = "source";
          };
          strictDeps = true;
          buildInputs = [ ];
          version = "0.1.0";
          pname = "app";
        };
      in
      rec {
        packages.default = craneLib.buildPackage commonArgs;

        devShell = craneLib.devShell {
          packages = with pkgs; [
            cargo-nextest
            cargo-outdated

            probe-rs-tools
          ];
        };
      }
    );
}
