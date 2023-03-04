#!/usr/bin/env bash
set -e

# Here's how to use Terraform without installing it :)
# Example:
# ./terraform.sh -chdir=02-aws-instance plan

docker run --rm -it \
    -v ~/.terraformrc:/root/.terraformrc:ro \
    --privileged=true -v /var/run/docker.sock:/var/run/docker.sock \
    -v ~/.docker/config.json:/root/.docker/config.json:ro \
    -v ~/.ssh:/root/.ssh:ro \
    -v $PWD:/ops -w /ops/ \
    --env-file=aws.env \
    hashicorp/terraform \
    $@

