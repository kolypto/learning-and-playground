variable "project_name" {
    description = "Name of the project to use. Lowercase."
    type = string
    default = "playground"
}

variable "app_docker_image_ecr_permissions" {
    description = "AWS users who can pull & push Docker images to the ECR intermediate registry. List of IAM ARNs."
    type = object({
        # List of user ARNs who can PUSH images to the server
        push_users = list(string)
        # List of server ARNs who can PULL images from the intermediate ECR registry
        pull_servers = list(string)
    })
    default = {
        # TODO: remove
        push_users = [
            "arn:aws:iam::352980582205:user/trygve@medthings.no",
            "arn:aws:iam::352980582205:user/mark@medthings.no",
        ]
        pull_servers = [
            "arn:aws:iam::352980582205:user/medthings-01",
        ]
    }
}
