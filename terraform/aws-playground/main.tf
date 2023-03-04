terraform {
  required_version = "~> 1.3.9"

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.57"
    }
  }
}

provider "aws" {
  region = "eu-central-1"
}


# List all availability zones in the current region
data "aws_availability_zones" "available" {
  state = "available"
}




locals {
    # Availability zones:
    # { 0 => { char: "a", name: "eu_central-1a"}, ... }
    availability_zones = [
        for i, az_name in sort(data.aws_availability_zones.available.names) :
            {
                index: i,  # index: 0, 1, ...
                char: substr("abcdefgh", i, 1), # char: "a", "b", ...
                name: az_name,  # az name: "eu_central-1a", ...
            }
    ]
}




output "result" {
  value = local.availability_zones
}
