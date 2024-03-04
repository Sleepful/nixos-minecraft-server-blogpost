# allow our nixpkgs import to be overridden if desired
{ pkgs ? import <nixpkgs> {}, ... }:

pkgs.rustPlatform.buildRustPackage {
  pname = "mc-watcher";
  version = "1.0.0";

  src = ./cargo;

  cargoLock = {
    lockFile = ./cargo/Cargo.lock;
  };

  meta = {
    mainProgram = "mc-watcher";
  };
}
