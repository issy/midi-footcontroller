{
  description = "My project";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=25.11";
    firmware.url = "path:./firmware";
    # proto.url = "path:./proto";
    # web.url = "path:./web";
  };

  outputs = { self, nixpkgs, firmware }:
    let
      system = builtins.currentSystem
      pkgs = import nixpkgs { inherit system; };
    in {
      packages.${system} = {
        firmware = firmware.packages.${system}.default;
        # proto = proto.packages.${system}.default;
        # web = web.packages.${system}.default;
      }
      # TODO: Add dev shells for each project
    };
}
