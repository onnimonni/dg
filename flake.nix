{
  description = "Decision Graph (dg) - Text-based knowledge graph for company decisions";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "dg";
          version = "0.1.0";
          src = ./.;
          cargoLock.lockFile = ./Cargo.lock;
        };
      }
    ) // {
      templates = {
        default = {
          path = ./templates/default;
          description = "Initialize a project with Decision Graph (dg) and Claude Code integration";
        };
      };

      # Devenv module for importing into other projects
      devenvModules.default = ./devenv-module.nix;
    };
}
