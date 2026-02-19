{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    nixpkgs = {
      # Last updated: 2026-02-19
      # New commit version can be found on https://github.com/NixOS/nixpkgs/tree/nixos-unstable-small
      url = "github:NixOS/nixpkgs/724cf38d99ba81fbb4a347081db93e2e3a9bc2ae";
    };
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        naersk-lib = pkgs.callPackage naersk { };
      in
      {
        defaultPackage = naersk-lib.buildPackage ./.;
        devShell = with pkgs; mkShell {
          buildInputs = [
            # Rust general
            cargo
            rustc
            rustfmt
            rustPackages.clippy
            # Dev Tools
            just
            prek

            # WASM
            lld
            wasm-pack
          ];
          RUST_SRC_PATH = rustPlatform.rustLibSrc;
        };
      }
    );
}
