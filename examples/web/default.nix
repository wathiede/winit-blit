let
  pkgs = import <nixpkgs> {
    overlays = [
      (import (builtins.fetchTarball
        "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"))
    ];
  };

  rust = pkgs.rust-bin.stable.latest.rust.override {
    extensions = [ "rust-src" ];
    targets = [ "wasm32-unknown-unknown" "x86_64-pc-windows-gnu" ];
  };

in with pkgs;
pkgs.mkShell rec {
  name = "rust";
  buildInputs = [
    cargo
    miniserve
    rust
    rust-analyzer
    rustfmt
    wasm-pack
  ];
}
