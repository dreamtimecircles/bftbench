{ pkgs ? import (builtins.fetchTarball {
           # Descriptive name to make the store path easier to identify
           name = "nixos-unstable-2023-05-18";
           # Commit hash for nixos-unstable as of 2023-05-18
           url = "https://github.com/nixos/nixpkgs/archive/cca2a7acb592a68c0ad6805e135ecb3b3baa6560.tar.gz";
           # Hash obtained using `nix-prefetch-url --unpack <url>`
           sha256 = "1dd9llizlnjk1z2hmxf44gxxxr41d0lfdmd2nla8f5pmk8pcndbg";
         }) {} }:

with pkgs;

mkShell {
  buildInputs = [
    protobuf
    git
    python310Packages.mdformat
    cargo
    rustc
    zlib
    zip
  ];
}
