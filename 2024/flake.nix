{
  description = "Advent of Code 2024";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs-mozilla.url = "github:mozilla/nixpkgs-mozilla";
    roc.url = "github:roc-lang/roc";
  };

  outputs = { self, nixpkgs, flake-utils, roc, nixpkgs-mozilla, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import nixpkgs-mozilla) ];
        };

        rocPkgs = roc.packages.${system};
        rocFull = rocPkgs.full;

        haskell = pkgs.haskellPackages.ghcWithPackages (pkgs: with pkgs; [ cabal-install ]);

        rusttoolchain = (pkgs.rustChannelOf {
          rustToolchain = ./rust-toolchain.toml;
          sha256 = "sha256-VZZnlyP69+Y3crrLHQyJirqlHrTtGTsyiSnZB8jEvVo=";
        }).rust;
      in
      {
        formatter = pkgs.nixpkgs-fmt;

        devShells = {
          default = pkgs.mkShell {
            nativeBuildInputs = with pkgs;
              [
                rocFull # includes CLI
                haskell
                haskell-language-server
                rusttoolchain
                zig
              ];

            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;

            shellHook = ''
              export ROC_LANGUAGE_SERVER_PATH=${rocFull}/bin/roc_language_server
            '';
          };
        };
      });
}
