# SSH into this server to push images
variable "server_ssh_connection_url" {
    description = "SSH connection url to a server to manage containers on: ssh://user@host"
    type = string

    validation {
        condition = startswith(var.server_ssh_connection_url, "ssh://")
        error_message = "Must start with ssh://"
    }
}

# Docker registry address to pull the image from
variable "docker_registry_address" {
    description = "Docker registry address to pull the image from"
    type = string
}

# The image to pull and deploy
variable "docker_image_name" {
    description = "Docker image to pull and deploy"
    type = string
}

# DB URLs for applications
variable "app_database_urls" {
    description = "DB URLs for our applications"
    type = object({
        goserver = string
    })
}