root:
	ssh -o UserKnownHostsFile=/dev/null root@${MC_IP}
	
user:
	ssh -o UserKnownHostsFile=/dev/null jose@${MC_IP}

build-image:
	docker build -t nixos/ec2-builder:arm64 . \
		--build-arg arch="arm64"

mc-build:
	docker run \
		--rm \
		--name mc-builder \
		--mount source=mcvol,target=/nix \
		-v ./scripts/.:/files/scripts/. \
		-v ./mc-server/.:/files/nix-files/. \
		--workdir /files \
		--env SSH_AUTH_SOCK="/run/host-services/ssh-auth.sock" \
		--env SERVER="${MC_IP}" \
		--env KEYNAME="MCServer" \
		-v /run/host-services/ssh-auth.sock:/run/host-services/ssh-auth.sock \
		-it \
		nixos/builder:arm64 bash -c "./scripts/build.sh ; bash"

mc-push:
	docker run \
		--rm \
		--name mc-builder \
		--mount source=mcvol,target=/nix \
		-v ./scripts/.:/files/scripts/. \
		-v ./mc-server/.:/files/nix-files/. \
		--workdir /files \
		--env SSH_AUTH_SOCK="/run/host-services/ssh-auth.sock" \
		--env SERVER="${MC_IP}" \
		--env KEYNAME="MCServer" \
		-v /run/host-services/ssh-auth.sock:/run/host-services/ssh-auth.sock \
		-it \
		nixos/builder:arm64 bash -c "./scripts/push.sh ; bash"


