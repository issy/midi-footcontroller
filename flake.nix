{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-25.11";
    flake-utils.url = "github:numtide/flake-utils";

    app-firmware = {
      url = "./firmware";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    app-web = {
      url = "./web";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };
  outputs = { self, nixpkgs, flake-utils, app-firmware, app-web }: flake-utils.lib.eachDefaultSystem (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      packages = {
        app-firmware = app-firmware.packages.${system}.default;
        app-web = app-web.packages.${system}.default;
      };

      devShell = pkgs.mkShell {
        buildInputs = with pkgs; [

        ];
      };
    }
  );
}
