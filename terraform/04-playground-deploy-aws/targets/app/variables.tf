# If you're tired of entering these values every time:
# 1. Use -var-file=../../playground.tfvars
# 2. Or createa a "playground.auto.tfvars" in the current folder


variable "app_docker_source_image_name" {
    description = "The source image to pull"
    type = string
}

variable "app_docker_ecr_registry_address" {
    description = "The ECT Registry to push the image to"
    type = string
}

variable "app_docker_image_ecr_permissions" {
    description = "AWS users who can pull & push Docker images to the ECR intermediate registry. List of IAM ARNs."
    type = object({
        # List of user ARNs who can PUSH images to the server
        push_users = list(string)
        # List of server ARNs who can PULL images from the intermediate ECR registry
        pull_servers = list(string)
    })
}

variable "app_docker_registry_names" {
    description = "Docker registries to use the credentials for (from your ~/.docker/config.json)"
    type = list(string)
}

# Remote state
variable "remote_state_s3_bucket" {
    description = "The bucket to read the remote state from"
    type = string
}
variable "remote_state_infrastructure" {
    description = "Path to the state file in the bucket"
    type = string
}
