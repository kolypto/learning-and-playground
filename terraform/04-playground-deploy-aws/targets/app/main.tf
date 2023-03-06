# This module will deploy the application on to the existing infrastructure


# * Create an ECR docker registry
# * Pull image from GitHub, push it to ECR
# * Start a container
# Prerequisites for pulling images from GitHub:
#   $ docker login ghcr.io -u kolypto -p 'ghp_...'
# Prerequisites for using ECR:
#   Add this line to ~/.docker/config.json:
#        "credHelpers": { "352980582205.dkr.ecr.eu-central-1.amazonaws.com": "ecr-login", "public.ecr.aws": "ecr-login" }
#   Now run:
#   $ apt install amazon-ecr-credential-helper
#   $ aws ecr get-login-password --region eu-central-1 | docker login --username AWS --password-stdin 352980582205.dkr.ecr.eu-central-1.amazonaws.com



# Load infrastructure data module: remote state
module "data" {
    source = "./remote-state"
    remote_state_s3_bucket = var.remote_state_s3_bucket
    remote_state_infrastructure = var.remote_state_infrastructure
}


# Set up the database
module "app_setup_database" {
    source = "./../../modules/app-setup-database"

    # Manage this instance
    postgres_url = module.data.infrastructure.postgres_psql_root
    project_name = module.data.infrastructure.project_name
    applications = ["goserver"]
}


# Push new Docker images
# Run only this target:
#   $ terraform -chdir=04-playground-deploy-aws apply -target=module.app_docker_image
module "app_docker_image" {
    source = "./../../modules/app-docker-image"

    # The image to push
    docker_image = var.app_docker_source_image_name

    # Name of the ECR image
    target_ecr_image_name = "${module.data.infrastructure.project_name}/app"

    # ECR Registry in the cloud
    ecr_registry_permissions = {
        push_users = var.app_docker_image_ecr_permissions.push_users
        pull_servers = var.app_docker_image_ecr_permissions.pull_servers
    }

    # Use our Docker config to sign into registries
    docker_auth_registry_names = var.app_docker_registry_names
}


# Deploy the docker container on the server
module "app_docker_deploy_container" {
    source = "./app-docker-containers"

    # Server to run the container on
    server_ssh_connection_url = "ssh://${module.data.infrastructure.server_ssh_user}@${module.data.infrastructure.server_public_ip}"

    # The image to deploy
    docker_registry_address = var.app_docker_ecr_registry_address # TODO: module.app_docker_image.docker_registry_url?
    docker_image_name = module.app_docker_image.pushed_image_name

    # DB URLs.
    # Note that key names in this parameter match those requested from "app_setup_database":
    # - "goserver"
    app_database_urls = module.app_setup_database.psql_applications
}
