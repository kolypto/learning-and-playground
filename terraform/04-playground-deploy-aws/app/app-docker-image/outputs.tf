output "deployed_image_id" {
    description = "Image id that we've deployed"
    # Image hash.
    # Example: "sha256:c5750c07180a4b35d0933f863c815d91fadd0664fbf2256f8c95ac8eae485d98"
    value = docker_image.pulled_source_image.image_id
}

output "deployed_image_name" {
    description = "Image tags: source and target (ECR)"
    value = {
        # Source image.
        # Example: "ghcr.io/playground/app:main"
        source_image = docker_image.pulled_source_image.name

        # Destination image in the ECR
        # Example: "123456.dkr.ecr.eu-central-1.amazonaws.com/playground/app:main"
        deployed_image = docker_registry_image.dst_ecr_image.name
    }
}
