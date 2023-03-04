# DB connection to manage
variable "postgres_url" {
    type = string
    description = "The DB to manage. Postgres connection url: postgres://user:password@host:port/. Provide AWS Instance URL"
}

# Project name. Used as DB name
variable "project_name" {
    type = string
    description = "Name of the project. Will be used as DB name"
}

# Application names.
# Every application gets their own login.
variable "applications" {
    type = list(string)
    description = "List of application names that will use the DB with their own accounts"
}
