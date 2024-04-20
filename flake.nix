{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/release-23.11";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix/monthly";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ fenix.overlays.default ];
        };
        inherit (pkgs) lib;
        toolchain = pkgs.fenix.fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-7QfkHty6hSrgNM0fspycYkRcB82eEqYa4CoAJ9qA3tU=";
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = [ toolchain ];
        };
      }
    );
}
