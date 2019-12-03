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
  cargoFile = ./Cargo.toml;
  system = builtins.currentSystem;
  cargo = pkgs.cargo;
  gcc = pkgs.gcc;
  coreutils = pkgs.coreutils;
  buildInputs = nixPackages;
  propagatedBuildInputs = [pkgs.pngquant];
}
