{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    clippy
  ];

  shellHook = ''
    echo "vim-navigator development environment"
    echo "Rust version: $(rustc --version)"
  '';
}
