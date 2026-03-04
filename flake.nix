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
        packages.default = pkgs.callPackage ./firmware/default.nix {};

        devShells.default = pkgs.mkShell {
          buildInputs = [
            (pkgs.rust-bin.stable."1.93.1".override {
              targets = ["riscv32imac-unknown-none-elf"];
              components = ["rust-src"];
            })
          ];
        };
      }
    );
}
