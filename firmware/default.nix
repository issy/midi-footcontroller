{ pkgs ? import <nixpkgs> {} }:

let
  rustToolchain = pkgs.rust-bin.stable."1.93.1".override {
    targets = ["riscv32imac-unknown-none-elf"];
    components = ["rust-src"];
  };
in

pkgs.stdenv.mkDerivation {
  pname = "footcontroller-firmware";
  version = "0.1.0";

  src = ./..;

  buildInputs = [ rustToolchain ];

  buildPhase = ''
    cd firmware
    cargo build --release
  '';

  installPhase = ''
    mkdir -p $out
    cp firmware/target/riscv32imac-unknown-none-elf/release/firmware $out/
  '';
}
