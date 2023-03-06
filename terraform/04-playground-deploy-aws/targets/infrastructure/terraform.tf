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

  # Save Terraform state into AWS S3
  # This makes the state shared. Allows to break everything into sub-modules.
  backend "s3" {
    # NOTE: Terraform will ask this value interactively!
    # You get it after you run the "init" target that creates a bucket for you.
    # bucket = "tfstate-2023..."

    key    = "playground/infrastructure"
    region = "eu-central-1"
  }
}

provider "aws" {
  # The region to use
  region = "eu-central-1"

  # Access key can be provided here
  # access_key = "my-access-key"
  # secret_key = "my-secret-key"

  # The provider can use credentials from ~/.aws/credentials and ~/.aws/config:
  # profile = "default"  # default profile name (from the credentials file)

  # Environment config:
  # $ export AWS_REGION="us-west-2"
  # $ export AWS_ACCESS_KEY_ID="anaccesskey"
  # $ export AWS_SECRET_ACCESS_KEY="asecretkey"

  # Environment, use config file:
  # $ export AWS_CONFIG_FILE=~/.aws/config
  # $ export AWS_SHARED_CREDENTIALS_FILE=~/.aws/credentials
  # $ export AWS_PROFILE="default"

  # If provided with a role ARN, assume this role
  # See blocks: `assume_role`, `assume_role_with_web_identity`,
}
