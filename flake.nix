{
  description = "Utility for copying contents directly from images";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, flake-utils, ... } @ inputs:
  let system = flake-utils.lib.system;
  in flake-utils.lib.eachSystem [
    system.x86_64-linux
    system.aarch64-linux
    system.aarch64-darwin
  ] (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
    in
  {
    packages.default = pkgs.rustPlatform.buildRustPackage {
      pname = "ocicp";
      version = "0.4.0";
      cargoLock.lockFile = ./Cargo.lock;
      src = pkgs.lib.cleanSource ./.;
    };
  });
}