terraform {
  required_version = "~> 1.3.9"

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.57"
    }
    docker = {
      source = "kreuzwerker/docker"
      version = "~> 3.0"
    }
  }

  backend "s3" {
    # NOTE: Terraform will ask this value interactively!
    # You get it after you run the "init" target that creates a bucket for you.
    # bucket = "tfstate-2023..."

    key    = "playground/app"
    region = "eu-central-1"
  }
}

provider "aws" {
  # The region to use
  region = "eu-central-1"
}
