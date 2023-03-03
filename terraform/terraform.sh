#!/usr/bin/env bash
set -e

# Example:
# ./terraform.sh -chdir=02-aws-instance plan

docker run --rm -it \
    -v ~/.terraformrc:/root/.terraformrc \
    --privileged=true -v /var/run/docker.sock:/var/run/docker.sock \
    -v ~/.docker/config.json:/root/.docker/config.json \
    -v $PWD:/ops -w /ops/ \
    --env-file=aws.env \
    hashicorp/terraform \
    $@

