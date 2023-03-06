# This module will:
# * Init an ECR registry
# * Pull image from a registry using local Docker
# * Push image to ECR
#
# Example workflow:
# * ECR registry: 123456.dkr.ecr.eu-central-1.amazonaws.com/playground/app
# * Source image: ghcr.io/company/app:main
# * Intermediate ECR image: 123456.dkr.ecr.eu-central-1.amazonaws.com/playground/app:main




# Create the intermediate ECR registry
# Note that one ECR can contain only one Docker image, so we create an ECR for the image
module "ecr_registry" {
  source = "./ecr-docker-registry"

  registry_name = var.target_ecr_image_name
  registry_aws_iam_arns = {
    push_users = var.ecr_registry_permissions.push_users
    pull_servers = var.ecr_registry_permissions.pull_servers
  }
}





# Pull local image ("src") and push it to the intermediate ECR repository ("dst")

# Get the latest image id from the source
data "docker_registry_image" "source_image" {
  provider = docker.local

  # The image to check
  name = var.docker_image
}


# Source: pull this image every time it's updated
# "docker_image": Pulls a Docker image to a given Docker host from a Docker Registry.
resource "docker_image" "pulled_source_image" {
  # Find and download this image
  provider = docker.local

  # The image to pull
  name = data.docker_registry_image.source_image.name

  # Keep the image up to date on the latest available version
  pull_triggers = [data.docker_registry_image.source_image.sha256_digest]

  # README: you can also use this resource to build an image. See `build`
}

locals {
  # ECR will have the same docker image tag as the source image
  # I.e. if it had ":main", the ECR image will also have ":main"
  ecr_docker_image_tag = split(":", data.docker_registry_image.source_image.name)[1]
}

# Retag the image: it will bear the name of the target registry.
# The name of the image defines which registry it will go to!
resource "docker_tag" "dst_ecr_image_tag" {
  provider = docker.local

  # Retag the image
  source_image = docker_image.pulled_source_image.image_id
  target_image = "${module.ecr_registry.docker_registry_url}:${local.ecr_docker_image_tag}"
}


# Target: Push it
# "docker_registry_image": Manage an image: e.g. push
resource "docker_registry_image" "dst_ecr_image" {
  # Push this image
  provider = docker.local
  name = docker_tag.dst_ecr_image_tag.target_image

  # On change, force push. Can be used to repush a local image (e.g. tag updated)
  triggers = {
    "sha256" : docker_tag.dst_ecr_image_tag.source_image_id
  }
}
