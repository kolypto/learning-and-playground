# Server IP
output "server_public_ip" {
    description = "Server IP address. You can SSH into it."
    value = module.server.server_public_ip
}

# Server SSH user
output "server_ssh_user" {
    description = "Server SSH user"
    value = module.server.server_ssh_user
}

# Database internal connection URL
output "postgres_psql_root" {
    description = "Postgres root user connection URL: postgres://user:pass@host:post/db"
    value = module.db_postgres.psql_internal_url
    sensitive = true
}


# Passthough some variables: just store them

# Project name
output "project_name" {
    description = "Project name (passthrough)"
    value = var.project_name
}

