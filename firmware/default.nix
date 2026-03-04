{ pkgs }:

pkgs.stdenv.mkDerivation {
  pname = "footcontroller-firmware";
  version = "0.1.0";

  src = ./..;

  buildInputs = [ pkgs.rust ];

  buildPhase = ''
    cd firmware
    cargo build --release --target riscv32imac-unknown-none-elf
  '';

  installPhase = ''
    mkdir -p $out
    cp firmware/target/riscv32imac-unknown-none-elf/release/firmware $out/
  '';
}
