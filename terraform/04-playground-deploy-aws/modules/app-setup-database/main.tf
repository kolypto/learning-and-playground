# This module will initialize the database for the app.
# It can connect directly to AWS RDS instances (!)
#
# It will create:
# * A database, named `var.project_name`
# * A user who owns this database, with the same name
# * A root user: `<database>-root`
# * For every application, a separate user with ALL permissions: `<database>-<application>`


# Database for the app
resource "postgresql_database" "app" {
  name = var.project_name  # db name
  owner = postgresql_role.owner.name  # only owner can drop it
}




# Root role: owns the database.
# Only they can make changes
resource "postgresql_role" "owner" {
  name = var.project_name
}

# Root user. Only they can make changes to the schema: e.g. migrations
resource "postgresql_role" "root_user" {
  name = "${var.project_name}-root"
  password = "${var.project_name}-root"
  roles = [postgresql_role.owner.name]
  login = true
}




# Application user: the application will use it to make queries.
# Separate user for every app is convenient in terms of logging & monitoring
resource "postgresql_role" "app_users" {
  name = "${var.project_name}-${each.value}"
  password = "${var.project_name}-${each.value}"  # TODO: perhaps a better password?
  login = true

  # Generate a user for every app
  for_each = toset(var.applications[*])
}

# Grant ALL privileges on this database
resource "postgresql_grant" "app_user_grants" {
  role = each.value
  object_type = "database"
  database = postgresql_database.app.name
  privileges = ["ALL"]

  # Generate a grant for every user
  for_each = toset([for user in postgresql_role.app_users: user.name])

  # Postgres provider doesn't like `privileges = ALL`:
  # every time it things that it changed to "CONNECT", "CREATE", "TEMPORARY"
  # Let's ignore it. Because it's already "ALL": can't get any bigger that this.
  lifecycle {
    ignore_changes = [privileges]  # Ignore changes to this attribute
  }
}







# Init provider: where to connect to?
provider "postgresql" {
    # use GoCloud to connect to AWS RDS instances (!)
    # Set endpoint value: host = "instance.xxx.region.rds.amazonaws.com"
    scheme   = "awspostgres"

    # This may workaround some issues with "Error: error detecting capabilities: error PostgreSQL version"
    # If it does not help, try: `$ terraform state rm 'module.app-setup-database.postgresql_database.app'`
    # expected_version = "15.2"

    # In Amazon RDS, we're not a superuser. Set to `false`: otherwise this error comes up:
    # > could not read role password from Postgres as connected user postgres is not a SUPERUSER
    superuser = false

    # Connect to
    host            = local.db_url.host
    port            = local.db_url.port
    username        = local.db_url.username
    password        = local.db_url.password

    # Timeout is small because we're fast :)
    connect_timeout = 15
}




locals {
  # Parse DB URL into an object: username, password, host, port[, database]
  db_url = regex(join("", [
      "(?:postgres|postgresql)?://?",  # postgres://, postgresql:/
      "(?P<username>.+?)", ":(?P<password>.+?)@",  # user:password@
      "(?P<host>.+?)", ":(?P<port>\\d+)", # host:port
      "(?:/(?P<database>.+))?",  # optional: /database
    ]), var.postgres_url)
}
