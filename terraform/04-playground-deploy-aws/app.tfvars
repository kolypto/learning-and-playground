# All the variables for this deployment in one place.
#
# Use this file like this:
#   $ terraform -chdir targets/infrastructure apply -var-file=../../app.tfvars
#
# Alternatively, you may export them as TF_VAR_* environment variables:
#   $ export TF_VAR_project_name=playground

# Project name
project_name = "playground"

# The port to open on the server
server_open_ports=[22, 80, 443, 8080]

# Remote state: S3 bucket to store the remote state to
remote_state_s3_bucket = "tfstate-20230306213054377200000001"
remote_state_infrastructure = "playground/infrastructure"

# Docker image to pull to push
app_docker_source_image_name = "ghcr.io/medthings/cerebellum-server:main"
app_docker_ecr_registry_address = "352980582205.dkr.ecr.eu-central-1.amazonaws.com"

# Docker ECR registry permissions: `push_users` will push images, `pull_servers` will pull them.
app_docker_image_ecr_permissions = {
    push_users = [
        "arn:aws:iam::352980582205:user/trygve@medthings.no",
        "arn:aws:iam::352980582205:user/mark@medthings.no",
    ]
    pull_servers = [
        "arn:aws:iam::352980582205:user/medthings-01",
    ]
}

# This Terraform module will use your Docker credentials to push images.
# Specify which ones.
app_docker_registry_names = ["ghcr.io", "352980582205.dkr.ecr.eu-central-1.amazonaws.com"]
