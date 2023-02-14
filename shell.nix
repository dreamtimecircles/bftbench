{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    git
    python310Packages.mdformat
    zlib
    zip
  ];
}
