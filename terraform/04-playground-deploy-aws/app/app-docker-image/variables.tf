# SSH into this server to push images
variable "server_ssh_connection_url" {
    description = "SSH connection url to a server to manage containers on: ssh://user@host"
    type = string

    validation {
        condition = startswith(var.server_ssh_connection_url, "ssh://")
        error_message = "Must start with ssh://"
    }
}


# ECR registry name and permissions
variable "ecr_registry" {
    description = "Intermediate ECR registry to push/pull the image through"
    type = object({
        name = string
        # These users can push images (users)
        # Example: "arn:aws:iam::352980582205:user/human"
        push_users = list(string)
        # These users can pull images (servers)
        # Example: "arn:aws:iam::352980582205:user/server"
        pull_servers = list(string)
    })
}


# The image to push to the server
variable "docker_image" {
    description = "The image to push"
    type = string
}
