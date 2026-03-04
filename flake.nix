{
  description = "Footcontroller-8 Firmware";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=25.11";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        rustToolchain = pkgs.rust-bin.stable."1.93.1".override {
          targets = ["riscv32imac-unknown-none-elf"];
          components = ["rust-src"];
        };
      in
      {
        packages.default = pkgs.stdenv.mkDerivation {
          pname = "footcontroller-firmware";
          version = "0.1.0";

          src = ./.;

          buildInputs = [ rustToolchain ];

          buildPhase = ''
            cd firmware
            cargo build --release
          '';

          installPhase = ''
            mkdir -p $out
            cp firmware/target/riscv32imac-unknown-none-elf/release/firmware $out/
          '';
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [ rustToolchain ];
        };
      }
    );
}
