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

  registry_name = var.ecr_registry.name
  registry_aws_iam_arns = {
    push_users = var.ecr_registry.push_users
    pull_servers = var.ecr_registry.pull_servers
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

























# module "cerebellum-server" {
#   source = "../modules/cerebellum-server"

#   env                   = local.env
#   db                    = local.db
#   docker-host           = data.terraform_remote_state.machine.outputs.public_ip
#   docker-image-server   = module.server-image.image
#   docker-image-ui       = module.ui-image.image
#   docker-image-rabbitmq = "rabbitmq:3.11.9-management"
# }


# data "docker_network" "bridge" {
#   name = "bridge"
# }

# data "docker_network" "traefik" {
#   name = "traefik"
# }

# data "docker_plugin" "loki" {
#   alias = "loki"
# }


# data "docker_registry_image" "cerebellum-ui" {
#   name = var.docker-image-ui
# }

# resource "docker_image" "cerebellum-ui" {
#   name          = var.docker-image-ui
#   pull_triggers = [data.docker_registry_image.cerebellum-ui.sha256_digest]
# }

# resource "docker_container" "cerebellum-ui" {
#   image    = docker_image.cerebellum-ui.image_id
#   name     = "cerebellum-ui"
#   must_run = false

#   networks_advanced {
#     name = data.docker_network.traefik.name
#   }

#   env = [
#     "API_URL=https://${aws_route53_record.a["device"].fqdn}/api/v1",
#   ]

#   labels {
#     label = "traefik.enable"
#     value = "true"
#   }

#   labels {
#     label = "traefik.http.routers.cerebellum-ui.rule"
#     value = "Host(`app.${var.env}.medthings.no`)"
#   }

#   labels {
#     label = "traefik.http.routers.cerebellum-ui.entrypoints"
#     value = "websecure"
#   }

#   labels {
#     label = "traefik.http.routers.cerebellum-ui.tls.certresolver"
#     value = "route53"
#   }

#   labels {
#     label = "traefik.http.services.cerebellum-ui.loadbalancer.server.port"
#     value = "4200"
#   }
# }
