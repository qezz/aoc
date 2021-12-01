let
  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
  rust-channel = (nixpkgs.rustChannelOf { date = "2021-11-02"; channel = "nightly"; });
  rust-nightly = rust-channel.rust.override {
    extensions = [
      "rust-src"
      "rls-preview"
      "rust-analysis"
      "rustfmt-preview"
    ];
  };
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "advent-of-code-2021";
    runtimeBuildInputs = [
      pkgconfig
      rust-analyzer
      tdlib
    ];
    buildInputs = [
      rust-nightly
      rust-analyzer
      pkgconfig
      openssl
      tdlib
    ];
  }
