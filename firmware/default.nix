{ pkgs ? import <nixpkgs> {} };

pkgs.stdenv.mkDerivation {
  pname = "web";
  version = "0.1.0";

  src = ./.;

  buildInputs = [
    pkgs.rust-bin.stable."1.93.1"
  ];

  buildPhase = ''
    echo "hello world"
  '';
}
