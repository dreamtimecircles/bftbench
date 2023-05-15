{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell {
  buildInputs = [
    protobuf
    git
    python310Packages.mdformat
    zlib
    zip
  ];
}
