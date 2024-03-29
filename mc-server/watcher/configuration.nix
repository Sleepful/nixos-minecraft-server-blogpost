{ lib, pkgs, ... }:
let 
 watcherPkg = pkgs.callPackage ./default.nix {};
in
{
  # adding the derivation to systemPackages makes it available to us
  environment.systemPackages = [ watcherPkg ];
  users.users.mc-watcher = {
    isSystemUser = true;
    extraGroups = [ "wheel" ];
    group = "mc-watcher";
  };
  users.groups.mc-watcher = {};

  systemd.services."mc-watcher" = {
    wantedBy = ["multi-user.target"];
    description = "watcher for Minecraft-server activity";
    serviceConfig = {
      Type = "simple";
      User = "mc-watcher";
      ExecStart = "${lib.getExe watcherPkg}";
    };
    path = [ "/run/wrappers" "/run/current-system/sw"];
  };
}
