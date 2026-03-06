{
  description = "Footcontroller-8 Firmware";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=25.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        packages.default = pkgs.stdenv.mkDerivation {
          pname = "footcontroller-firmware";
          version = "0.1.0";

          src = ./.;

          buildInputs = [ pkgs.rustup pkgs.rustc ];

          buildPhase = ''
            export CARGO_HOME="$NIX_BUILD_TOP/.cargo"
            export RUSTUP_HOME="$NIX_BUILD_TOP/.rustup"
            cd firmware
            cargo build --release --target riscv32imac-unknown-none-elf
          '';

          installPhase = ''
            mkdir -p $out
            cp target/riscv32imac-unknown-none-elf/release/firmware $out/
          '';
        };

        devShells.default = pkgs.mkShell {
          buildInputs = [ pkgs.rustup pkgs.rustc ];
        };
      }
    );
}
