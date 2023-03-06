variable "project_name" {
  type = string
  description = "Name of the project. Will be used as the DB server name"
}


variable "vpc_id" {
    type = string
    description = "VPC to create the database in. Used to configure subnet security groups"
}


variable "subnet_ids" {
    type = list(string)
    description = "Subnet IDs that the Database should be made available in. Must be 2+"
    validation {
      condition = length(var.subnet_ids) >= 2
      error_message = <<-EOF
        AWS limitation: an RDS instance must be in 2 or more different availability zones.
        Please provide at least two subnets in different availability zones
      EOF

      # Here's how the error message looks like:
      # > │ Error: creating RDS DB Subnet Group (db-subnet-2023...): DBSubnetGroupDoesNotCoverEnoughAZs:
      # The DB subnet group doesn't meet Availability Zone (AZ) coverage requirement.
      # Current AZ coverage: eu-central-1a. Add subnets to cover at least 2 AZs.
    }
}
