{
  description = "Status flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    fenix = {
      url = "github:nix-community/fenix/staging";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix }:
  let
    overlays = [ fenix.overlays.default ];
    systems = [
      "x86_64-linux"
      "x86_64-darwin"
      "aarch64-linux"
      "aarch64-darwin"
    ];

    getPkgsFor = system: import nixpkgs {
      inherit system overlays;
    };

    forEachSystem = (func:
      nixpkgs.lib.genAttrs systems (system:
        func (getPkgsFor system)
      )
    );
  in

  {
    devShells = forEachSystem (pkgs: {
      default = pkgs.callPackage ./shell.nix {};
    });
  };
}
