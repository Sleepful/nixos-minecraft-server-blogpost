# we add our server to known_hosts to avoid the fingerprint message when we send our
# build to the server through SSH
echo `ssh-keyscan -t rsa $SERVER` > ~/.ssh/known_hosts
# and the public key is attached to the nix build, so that we can login into the user
# account with the same keypair as we do to the root account
echo `ssh-add -L  | grep $KEYNAME` > /public.key
# finally we build the config and store the path to the derivation in a file,
# this path is used later to know where to copy the nix build
nix-build --show-trace \
  ./nix-files/server.nix >> /store.path
