{nixpkgs ? import <nixpkgs> {}}:
let
  inherit (nixpkgs) pkgs;
  nixPackages = [
    pkgs.rustc
    pkgs.cargo
  ];
in
pkgs.stdenv.mkDerivation {
  name = "env";
  buildInputs = nixPackages;
}
