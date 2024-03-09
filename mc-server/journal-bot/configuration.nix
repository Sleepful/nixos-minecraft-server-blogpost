{ lib, pkgs, ... }:
let 
 journalBotPkg = pkgs.callPackage ./default.nix {};
 notEmpty = env:
    assert env != "";
    env;
 discordURL = notEmpty builtins.readFile /env-vars/DISCORD_WEBHOOK_URL;
in
{
  # adding the derivation to systemPackages makes it available to us
  environment.systemPackages = [ journalBotPkg ];
  users.users.journal-bot = {
    isSystemUser = true;
    extraGroups = [ "wheel" ];
    group = "journal-bot";
  };
  users.groups.journal-bot = {};

  systemd.services."journal-bot" = {
    wantedBy = ["multi-user.target"];
    description = "Sends journal entries to Discord channel defined by DISCORD_WEBHOOK_URL";
    serviceConfig = {
      Type = "simple";
      User = "journal-bot";
      ExecStart = "${lib.getExe journalBotPkg}";
    };
    path = [ "/run/current-system/sw/bin/journalctl"];
    environment = {
      DISCORD_WEBHOOK_URL = discordURL; 
    };
  };
}
