{nixpkgs ? import <nixpkgs> {}}:
let
  inherit (nixpkgs) pkgs;
  nixPackages = [
    pkgs.rustc
    pkgs.cargo
  ];
in
pkgs.stdenv.mkDerivation {
  name = "aoc";
  builder = "${pkgs.bash}/bin/bash";
  args = [ ./builder.sh ];
  src = ./main.rs;
  system = builtins.currentSystem;
  rustc = pkgs.rustc;
  gcc = pkgs.gcc;
  coreutils = pkgs.coreutils;
  buildInputs = nixPackages;
}
