terraform {
  required_providers {
    docker = {
      source = "kreuzwerker/docker"
      version = "~> 3.0"
    }
  }
}



# The Docker daemon running on the remote server
provider "docker" {
    # Docker host: connect to Docker via SSH
    host = var.server_ssh_connection_url
    ssh_opts = ["-o", "StrictHostKeyChecking=no", "-o", "UserKnownHostsFile=/dev/null"]

    # How to authenticate into the Docker registry?
    registry_auth {
        # Auth using config file (default)
        # You can also use: $DOCKER_CONFIG to provide a different Docker config
        # You can also use: $DOCKER_REGISTRY_USER, $DOCKER_REGISTRY_PASS
        address = var.docker_registry_address
    }

    # # Examples:
    # registry_auth {
    #     # Example: config file auth
    #     # NOTE: credentials from the config file have precedence! They will override any login/passwords!
    #     address     = "registry-1.docker.io"
    #     config_file = pathexpand("~/.docker/config.json") # Or use: `config_file_content`
    # }
    # registry_auth {
    #     # Example: username/password auth
    #     # You can also use: $DOCKER_REGISTRY_USER, $DOCKER_REGISTRY_PASS
    #     address  = "quay.io:8181"
    #     username = "someuser"
    #     password = "somepass"
    # }
}
