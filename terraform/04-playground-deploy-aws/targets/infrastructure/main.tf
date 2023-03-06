# This module will bring the infrastructure up
# * Create an EC2 server
# * Create an RDS Postgres database

# First run: init S3 bucket for renote state:
#   $ terraform -chdir targets/infrastructure init -backend=false
#   $ terraform -chdir targets/infrastructure apply -target=module.remote_state
#   $ terraform -chdir targets/infrastructure init -reconfigure
# Now feel free to:
#   $ terraform apply
#
# Make sure you have the environment configured:
#   $ export AWS_REGION="us-west-2"
#   $ export AWS_ACCESS_KEY_ID="anaccesskey"
#   $ export AWS_SECRET_ACCESS_KEY="asecretkey"


# Create the server and its network
module "server" {
    source = "./../../modules/server-with-network"

    # NOTE: we do not need to initialize providers within a module:
    # because providers from the root module propagate into other modules!
    project_name = var.project_name
    server_name = var.project_name
    server_open_ports = var.server_open_ports

    # The SSH public key we want to use for it
    ssh_public_key_file = pathexpand("~/.ssh/id_rsa.pub")
}


# Create a database
module "db_postgres" {
    source = "./../../modules/server-rds-postgres"

    # Put it into the same subnets the server is in
    # NOTE: AWS requires that an RDS instance is in at least 2 availability zone subnets!
    project_name = var.project_name
    vpc_id = module.server.vpc_id
    subnet_ids = module.server.vpc_server_subnet_ids

    # Experimental.
    # Postgres needs a server for GoCloud to use as a proxy. If the server's missing, we can't connect.
    depends_on = [module.server]
}
