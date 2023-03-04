output "psql_internal_url" {
    description = "Postgres connection URL, admin user, internal"
    value = format(
        "%s://%s:%s@%s:%s/%s",
        aws_db_instance.db.engine,
        aws_db_instance.db.username, aws_db_instance.db.password,
        aws_db_instance.db.endpoint, aws_db_instance.db.port,
        aws_db_instance.db.db_name
    )
}
