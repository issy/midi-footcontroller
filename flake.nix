{
  description = "My project";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=25.11";
  };

  outputs = { self, nixpkgs }:
    let
      system = "aarch64-darwin";
      pkgs = import nixpkgs { inherit system; };
    in {
      packages.${system} = {
        packages.hello = pkgs.hello;
      };
    };
}
