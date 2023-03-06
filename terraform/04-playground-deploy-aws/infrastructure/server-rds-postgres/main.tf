# This module will create:
# * Postgres instance in a subnet
# * One root user with random password





# Get a Postgres database.
# See also: "aws_db_instance_automated_backups_replication"
resource "aws_db_instance" "db" {
    engine = "postgres"
    engine_version = "15.2"
    identifier_prefix = "${var.project_name}-db-"

    # Postgres
    username = "postgres"
    password = random_password.db_password.result
    db_name  = "postgres"  # create a database
    parameter_group_name = aws_db_parameter_group.db_params.name

    # Instance
    instance_class    = "db.t3.micro"
    storage_encrypted = true  # Encypt data on disk. Default: false
    publicly_accessible = true  # Is it publicly accessible? Default: false

    # Name
    tags = { Name = "${var.project_name} db" }

    # Network
    db_subnet_group_name   = aws_db_subnet_group.db_subnet.name
    vpc_security_group_ids = [aws_security_group.db.id]

    # Management
    apply_immediately = false  # Do not wait for a maintenance window, apply changes immediately. Default: false
    deletion_protection = false  # The DB cannot be deleted. Default: false
    delete_automated_backups = true  # Delete backups when the DB is deleted. Default: true
    skip_final_snapshot = true  # Do not make a final snapshot before removing it. Default: false
    performance_insights_enabled = true  # Watch performance. Default: false

    # Maintenance window for: backups and upgrades
    maintenance_window = "Mon:01:00-Mon:03:00"  # UTC
    backup_window      = "00:00-00:59"          # UTC
    backup_retention_period = 3  # Keep backups for N days. Default: 0 = disabled
    copy_tags_to_snapshot = true  # Copy all instance Tags to snapshots. Default: false

    # Upgrades during the maintenance window
    auto_minor_version_upgrade = true   # Auto upgrade minor versions. Default: true
    allow_major_version_upgrade = true  # Allow major upgrades. Default: false?

    # Autoscaling of the hard drive
    allocated_storage     = 10   # Gb of disk space
    max_allocated_storage = 100  # Auto-scale disk space up to this many Gb
}



# Generate a random password.
# Once generated, it will remain.
resource "random_password" "db_password" {
    length = 16
    special = false
}


# Parameters
resource "aws_db_parameter_group" "db_params" {
    name_prefix = "${var.project_name}-db-params-"
    family = "postgres15"

    # Parameters: https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Appendix.PostgreSQL.CommonDBATasks.Parameters.html
    parameter {
        name  = "log_connections"
        value = "1"
    }

    # Name
    tags = { Name = "${var.project_name} db" }
}




# NOTE: Terraform may PLAN a change for the next maintenance window!

# NOTE: See RDS Blue/Green deployments for low downtime updates: (only for MySQL/MariaDB)
#   https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/blue-green-deployments.html
