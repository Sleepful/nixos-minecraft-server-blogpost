root:
	ssh -o StrictHostKeyChecking=accept-new root@${MC_IP}
	
user:
	ssh -o StrictHostKeyChecking=accept-new jose@${MC_IP}

turn-on:
	aws ec2 start-instances --no-cli-pager --instance-ids ${MC_INSTANCE_ID}

shut-down:
	aws ec2 stop-instances --no-cli-pager --instance-ids ${MC_INSTANCE_ID}

status:
	aws ec2 describe-instances --no-cli-pager --instance-ids ${MC_INSTANCE_ID} | jq -r '.Reservations[].Instances[].State.Name'

build-image:
	docker build -t nixos/ec2-builder:arm64 . \
		--build-arg arch="arm64"

mc-builder:
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
		--env DISCORD_WEBHOOK_URL="${DISCORD_WEBHOOK_URL}" \
		-v /run/host-services/ssh-auth.sock:/run/host-services/ssh-auth.sock \
		-it \
		nixos/builder:arm64 bash

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
		--env DISCORD_WEBHOOK_URL="${DISCORD_WEBHOOK_URL}" \
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
		--env DISCORD_WEBHOOK_URL="${DISCORD_WEBHOOK_URL}" \
		-v /run/host-services/ssh-auth.sock:/run/host-services/ssh-auth.sock \
		-it \
		nixos/builder:arm64 bash -c "./scripts/push.sh ; bash"


