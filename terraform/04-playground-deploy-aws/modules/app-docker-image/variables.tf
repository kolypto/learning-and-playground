
# The image to push to the server
variable "docker_image" {
    description = "The image to push"
    type = string
}


# Docker authentication
variable "docker_auth_registry_names" {
    description = <<-EOF
        Docker auth key names to use -- from your ~/.docker/config.json.
        It needs to have access both to the source and the target registries
    EOF
    type = list(string)
}



# The ECR registry name
variable "target_ecr_image_name" {
    description = "ECR registry name for the image. Feel free to use /"
    type = string
}

# ECR registry name and permissions
variable "ecr_registry_permissions" {
    description = "Intermediate ECR registry to push/pull the image through"
    type = object({
        # These users can push images (users)
        # Example: "arn:aws:iam::352980582205:user/human"
        push_users = list(string)

        # These users can pull images (servers)
        # Example: "arn:aws:iam::352980582205:user/server"
        pull_servers = list(string)
    })
}
