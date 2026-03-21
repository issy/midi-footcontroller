{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        commonArgs = {
          src = pkgs.lib.cleanSourceWith {
            src = ./.;
            name = "source";
          };
          strictDeps = true;
          buildInputs = with pkgs; [
            nodejs_24
            pnpm
          ];
          nativeBuildInputs = with pkgs; [
            nodejs_24
            pnpm
          ];
          buildPhase = ''
            pnpm install --frozen-lockfile
            pnpm build
          '';
          installPhase = ''
            mkdir -p $out
            cp -r dist/* $out/
          '';
          version = "0.1.0";
          pname = "app-web";
        };
      in
      rec {
        packages.default = pkgs.stdenv.mkDerivation commonArgs;

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            nodejs_24
            pnpm
          ];
          shellHook = ''
          echo "📦 Installing dependencies..."
          pnpm install --frozen-lockfile --quiet
          '';
        };
      }
    );
}
