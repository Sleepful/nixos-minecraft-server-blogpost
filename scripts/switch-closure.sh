# activate the nixos
STORE_PATH=$(cat /store.path) \
  SERVER_PROFILE="/nix/var/nix/profiles/system" \
  && ssh root@$SERVER \
    "nix-env --profile $SERVER_PROFILE --set $STORE_PATH;" \
    "$SERVER_PROFILE/bin/switch-to-configuration switch"
