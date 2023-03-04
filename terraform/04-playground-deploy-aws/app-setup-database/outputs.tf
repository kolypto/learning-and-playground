
# Root user
output "psql_root" {
    description = "Postgres connection URL: connect as root user (to run migrations)"
    value = format(
        "postgresql://%s:%s@%s:%s/%s",
        postgresql_role.root_user.name, postgresql_role.root_user.password,
        local.db_url.host, local.db_url.port,
        postgresql_database.app.name
    )
    sensitive = true
}


# Application users
output "psql_applications" {
    description = "Postgres connection URL: for each of your applications"
    value = {
        for app_name, app_user in postgresql_role.app_users:
            app_name => format(
                "postgresql://%s:%s@%s:%s/%s",
                app_user.name, app_user.password,
                local.db_url.host, local.db_url.port,
                postgresql_database.app.name
            )
    }
    sensitive = true
}
