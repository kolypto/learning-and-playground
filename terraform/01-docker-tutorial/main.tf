terraform {
  # Terraform version
  required_version = "~> 1.3"

  # Provider: use Docker
  required_providers {
    docker = {
      source  = "kreuzwerker/docker"
      version = "~> 3.0.1"
    }
  }
}

# Configure provider
provider "docker" {}

# Docker image
resource "docker_image" "nginx" {
  name         = "nginx:latest"
  keep_locally = false
}

# Docker container
# Use `terraform show` to fin all the values
resource "docker_container" "nginx" {
  image = docker_image.nginx.image_id
  name  = "tutorial"
  ports {
    internal = 80
    external = 8000
  }

}


# Now:
# $ terraform plan
# $ terraform apply
# $ terraform destroy
