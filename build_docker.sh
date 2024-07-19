#!/usr/bin/env bash

# Exit script as soon as a command fails.
set -ex

# Predefined ANSI escape codes for colors
YELLOW='\033[0;33m'
NC='\033[0m'

NAME="small-proxy-factory"

# Switch to current directory
pushd $(dirname ${BASH_SOURCE[0]})

# Pick the correct tag to pull from Docker Hub based on OS architecture
_warning="
${YELLOW}WARNING${NC}: You are building smart contracts using ARM64. The resulting artifacts will
be usable for testing, but won't match amd64 builds.
"
if [[ $(uname -m) == 'arm64' ]]; then
    echo -e "$_warning"
    TAG="latest-arm64"
else
    TAG="latest-amd64"
fi

if docker ps -a --format '{{.Names}}' | grep -Eq "^build_${NAME}\$"; then
    echo "Container exists"
else
    docker create \
        --mount type=bind,source=$(pwd),target=/host \
        --cap-add=SYS_PTRACE --security-opt seccomp=unconfined \
        --name=build_"$NAME" \
        -w /host/"$NAME" \
        -e RUSTFLAGS='-C link-arg=-s' \
        -it nearprotocol/contract-builder:"$TAG" \
        /bin/bash
fi

docker start build_"$NAME"
docker exec build_"$NAME" /bin/bash -c "./build.sh"
