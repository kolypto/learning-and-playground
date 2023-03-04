
# === Infrastructure first === #

# Create the server and its network
module "server" {
    source = "./server-with-network"

    # NOTE: we do not need to initialize providers within a module:
    # because providers from the root module propagate into other modules!

    project_name = "playground"
    server_name = "playground"
}

# Create a database
module "db_postgres" {
    source = "./server-rds-postgres"

    # Put it into the same subnets the server is in
    # NOTE: AWS requires that an RDS instance is in at least 2 availability zone subnets!
    vpc_id = module.server.vpc_id
    subnet_ids = module.server.vpc_server_subnet_ids
}

# === Now deploy the app to this infrastructure === #

# Set up the database
module "app-setup-database" {
    source = "./app-setup-database"

    # Manage this instance
    postgres_url = module.db_postgres.psql_internal_url
    project_name = "playground"
    applications = ["goserver"]
}
