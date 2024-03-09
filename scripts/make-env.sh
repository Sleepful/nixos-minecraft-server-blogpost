# this shell records env vars to files, in order for our configuration to find them during `nix-build`
# for example, when we use `builtins.readFile /env-vars/DISCORD_WEBHOOK_URL` in a .nix config file.
mkdir /env-vars
echo $DISCORD_WEBHOOK_URL > /env-vars/DISCORD_WEBHOOK_URL
