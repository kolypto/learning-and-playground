# Global config
terraform {
  # Version constraints
  # Operators:
  # * "0.15.0" static
  # * ">= 0.15" any version greater than this one
  # * "~> 0.15.0" any version 0.15.x. The operator allows only the (!) rightmost version component to increment.
  # * ">= 0.15, < 2.0.0" specific
  # Best practice: "~>"
  required_version = "~> 1.3.5"

  # Providers to install
  required_providers {
    # Manage Docker images & containers
    docker = {
      source = "kreuzwerker/docker"
      version = "~> 3.0.1"
    }

    # Generate words and ids
    random = {
      source = "hashicorp/random"
      version = "3.1.0"
    }

    # Count
    time = {
      source  = "hashicorp/time"
      version = "~> 0.7.2"
    }
  }
}
