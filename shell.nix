{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustup
  ];

  shellHook = ''
    export RUST_LOG=debug
  '';
}
