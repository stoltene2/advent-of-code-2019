{nixpkgs ? import <nixpkgs> {}}:
let
  inherit (nixpkgs) pkgs;
  nixPackages = [
    pkgs.rustc
    pkgs.cargo
    pkgs.rustfmt
  ];
in
pkgs.stdenv.mkDerivation {
  name = "env";
  buildInputs = nixPackages;
}
