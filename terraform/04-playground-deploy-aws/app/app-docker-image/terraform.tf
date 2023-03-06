terraform {
  required_providers {
    docker = {
      source = "kreuzwerker/docker"
      version = "~> 3.0"
    }
  }
}



# Our local Docker daemon.
# We'll use it to pull & push images
provider "docker" {
    alias = "local"

    # Docker host: connect to local Docker
    # host = "unix:///var/run/docker.sock"

    # Pull images from a remote repository (github) using our local Docker
    registry_auth {
        address = "ghcr.io"
    }
    # Push images to this repository using our local Docker
    registry_auth {
        address = "352980582205.dkr.ecr.eu-central-1.amazonaws.com"
    }
}


# The target repository to copy the image to
# It should be accessible from your server
provider "docker" {
    alias = "server"

    # Docker host: connect to Docker via SSH
    host = var.dst_ssh_connection_url
    ssh_opts = ["-o", "StrictHostKeyChecking=no", "-o", "UserKnownHostsFile=/dev/null"]

    # How to authenticate into the Docker registry?
    registry_auth {
        # Auth using config file (default)
        # You can also use: $DOCKER_CONFIG to provide a different Docker config
        # You can also use: $DOCKER_REGISTRY_USER, $DOCKER_REGISTRY_PASS
        address = local.dst_repository_address

        # Use specific config file
        # config_file = ...  # Default: "~/.docker/config.json"
    }






    # Examples:
    registry_auth {
        # Example: config file auth
        # NOTE: credentials from the config file have precedence! They will override any login/passwords!
        address     = "registry-1.docker.io"
        config_file = pathexpand("~/.docker/config.json") # Or use: `config_file_content`
    }
    registry_auth {
        # Example: username/password auth
        # You can also use: $DOCKER_REGISTRY_USER, $DOCKER_REGISTRY_PASS
        address  = "quay.io:8181"
        username = "someuser"
        password = "somepass"
    }

}
