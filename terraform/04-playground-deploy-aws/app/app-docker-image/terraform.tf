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
    dynamic "registry_auth" {
        for_each = var.docker_auth_registry_names
        content {
          address = setting.value
        }
    }

    # TODO: we can pull the image directly from the server by using our local Docker config, like this:
    # registry_auth {
    #   address = "ghcr.io"
    #   config_file_content = file(....)
    # }
}
