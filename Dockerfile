# arch is the architecture
ARG arch
FROM nixos/nix:latest-${arch}

WORKDIR /files/image

# this is helpful to look at the platform you are building on,
# all it does is print the result to "gnu-config" file in the container.
RUN bash $(nix-build '<nixpkgs>' -A gnu-config)/config.guess > gnu-config

# filter-syscalls is necssary to avoid a weird error
# https://github.com/NixOS/nix/issues/5258
RUN echo "filter-syscalls = false" >> /etc/nix/nix.conf

# get latest nixpkgs-unstable during building, to update the channels at 
# a later time inside the container this needs to be executed again
RUN nix-channel --update
RUN nix-channel --list 

# copy the base config into docker
COPY ./ec2-base.nix /files/image/ec2-base.nix

# this path is the pkgs path in the host computer, in this case MacOS
ARG PKGS=/nix/var/nix/profiles/per-user/root/channels/nixpkgs
RUN nix-build -I nixpkgs=`realpath $PKGS` --show-trace ec2-base.nix
