output "pushed_image_id" {
    description = "Image id that we've deployed"
    # Image hash.
    # Example: "sha256:c5750c07180a4b35d0933f863c815d91fadd0664fbf2256f8c95ac8eae485d98"
    value = docker_image.pulled_source_image.image_id
}

output "pushed_image_name" {
    description = "Pushed image name in the ECR repository"
    # Image name.
    # Example: "123456.dkr.ecr.eu-central-1.amazonaws.com/playground/app:main"
    value = docker_registry_image.dst_ecr_image.name
}


# Docker registry URL.
# You will need it to configure Docker pulling with provider "docker" { registry_auth { ... } }
output "docker_registry_url" {
    description = "Docker URL for this ECR registry"
    value = module.ecr_registry.docker_registry_url
}
