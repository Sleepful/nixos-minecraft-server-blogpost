# allow our nixpkgs import to be overridden if desired
{ pkgs ? import <nixpkgs> {}, ... }:

pkgs.rustPlatform.buildRustPackage {
  pname = "journal-bot";
  version = "1.0.0";

  src = ./cargo;

  cargoLock = {
    lockFile = ./cargo/Cargo.lock;
  };

  meta = {
    mainProgram = "journal-bot";
  };
}
