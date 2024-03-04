let
  nixos = import <nixpkgs/nixos> {
    configuration = { modulesPath, ... }:
    {
      imports = [ "${modulesPath}/virtualisation/amazon-image.nix" ];
      networking.hostName = "ec2-NixOS";
    };
  };
in
  nixos.system
