terraform {
    required_providers {
        # Used to generate a random password for Postgres
        random = {
            source = "hashicorp/random"
            version = "~> 3.4"
        }
    }
}
