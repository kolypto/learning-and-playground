# This module will:
# * Create an EC2 server
# * Create an RDS Postgres database
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



# === Infrastructure first === #

# Create the server and its network
module "server" {
    source = "./infrastructure/server-with-network"

    # NOTE: we do not need to initialize providers within a module:
    # because providers from the root module propagate into other modules!
    project_name = var.project_name
    server_name = var.project_name
    server_open_ports = [80,443,8888]

    # The SSH public key we want to use for it
    ssh_public_key_file = pathexpand("~/.ssh/id_rsa.pub")
}

# Create a database
module "db_postgres" {
    source = "./infrastructure/server-rds-postgres"

    # Put it into the same subnets the server is in
    # NOTE: AWS requires that an RDS instance is in at least 2 availability zone subnets!
    project_name = var.project_name
    vpc_id = module.server.vpc_id
    subnet_ids = module.server.vpc_server_subnet_ids

    # Experimental.
    # Postgres needs a server for GoCloud to use as a proxy. If the server's missing, we can't connect.
    depends_on = [module.server]
}

# === Now deploy the app to this infrastructure === #

# Set up the database
module "app_setup_database" {
    source = "./app/app-setup-database"

    # Manage this instance
    postgres_url = module.db_postgres.psql_internal_url
    project_name = var.project_name
    applications = ["goserver"]
}


# Push new Docker images
# Run only this target:
#   $ terraform -chdir=04-playground-deploy-aws apply -target=module.app_docker_image
module "app_docker_image" {
    source = "./app/app-docker-image"

    # The image to push
    docker_image = "ghcr.io/medthings/cerebellum-server:main"

    # Name of the ECR image
    target_ecr_image_name = "${var.project_name}/app"

    # ECR Registry in the cloud
    ecr_registry_permissions = {
        push_users = var.app_docker_image_ecr_permissions.push_users
        pull_servers = var.app_docker_image_ecr_permissions.pull_servers
    }

    # Use our Docker config to sign into registries
    docker_auth_registry_names = ["ghcr.io", "352980582205.dkr.ecr.eu-central-1.amazonaws.com"]
}


# Deploy the docker container on the server
module "app_docker_deploy_container" {
    source = "./app/app-docker-containers"

    # Server to run the container on
    server_ssh_connection_url = "ssh://${module.server.server_ssh_user}@${module.server.server_public_ip}"

    # The image to deploy
    docker_registry_address = "352980582205.dkr.ecr.eu-central-1.amazonaws.com" #module.app_docker_image.docker_registry_url
    docker_image_name = module.app_docker_image.pushed_image_name

    # DB URLs.
    # Note that key names in this parameter match those requested from "app_setup_database":
    # - "goserver"
    app_database_urls = module.app_setup_database.psql_applications
}
