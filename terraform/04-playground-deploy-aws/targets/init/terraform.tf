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
