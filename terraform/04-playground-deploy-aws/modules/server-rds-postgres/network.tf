# Create Postgres in the following subnet
# RDS instance will be created in the VPC this subnet belongs to
resource "aws_db_subnet_group" "db_subnet" {
  name_prefix = "db-${var.project_name}-subnet-"

  # List of VPC subnet ids to make the DB available in
  # Example: subnet_ids = [aws_subnet.frontend.id, aws_subnet.backend.id]
  subnet_ids = var.subnet_ids

  tags = { Name = "${var.project_name} DB Subnet" }
}


# Configure security groups for Postgres
resource "aws_security_group" "db" {
    # Name prefix: use it to make sure names stay unique
    name_prefix   = "db-${var.project_name}-sg-"

    # VPC to define it on
    vpc_id = var.vpc_id

    # Name
    tags = { Name = "${var.project_name} DB Security" }
    description = "DB security: allow PostgreSQL in, nothing out"
}

# Postgres security group: allow incoming Postgres connections
resource "aws_vpc_security_group_ingress_rule" "db_in_postgres" {
    security_group_id = aws_security_group.db.id

    description = "Postgres in"
    cidr_ipv4   = "0.0.0.0/0"
    ip_protocol = "tcp"
    from_port   = 5432
    to_port     = 5432
}
