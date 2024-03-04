# This is the script that gets ran from within the docker container.
# The paths are relative to the /files directory in the docker container.

./scripts/build.sh
./scripts/copy-closure.sh
./scripts/switch-closure.sh
