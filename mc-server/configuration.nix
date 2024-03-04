{ pkgs, modulesPath, lib, ... }:

let 
  system = "${modulesPath}/virtualisation/amazon-image.nix";
  watcher = ./watcher/configuration.nix;
  imports = [ system watcher ];
in
{
  # == system ==
  imports = imports;
  networking.hostName = "MC-Server";
  services.openssh.settings.PasswordAuthentication = false;
  services.fail2ban.enable = true;

  security.sudo.wheelNeedsPassword = false;
  nix.settings.trusted-users = [ "@wheel" ];
  users.users.jose = {
    isNormalUser = true;
    extraGroups = [ "wheel" ];
    openssh.authorizedKeys.keyFiles = [ /public.key ];
  };

  environment.systemPackages = [ pkgs.vim pkgs.htop pkgs.netcat pkgs.tree pkgs.cloud-utils ];
  services.journald.extraConfig = ''
    SystemMaxUse=300M
  '';

  # ==  minecraft-server  ==
  services.minecraft-server = {
    enable = true;
    eula = true;
    openFirewall = true;
    # declarative = true;
    # white-list = {};
    serverProperties = {
      server-port = 25565;
      difficulty = 3;
      gamemode = 1;
      max-players = 5;
      motd = "NixOS Minecraft server!";
      # white-list = true;
    };
    jvmOpts = "-Xms1024M -Xmx3072M";
  };
  
  nixpkgs.config.allowUnfreePredicate = pkg: builtins.elem (lib.getName pkg) [
             "minecraft-server"
           ];
}
