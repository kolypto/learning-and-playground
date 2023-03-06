# Data module.
# It only reports data about the current infrastructure.

data "terraform_remote_state" "infrastructure" {
    backend = "s3"
    config = {
        bucket = var.remote_state_s3_bucket
        key    = var.remote_state_infrastructure
        region = data.aws_region.current.name
    }
}

data "aws_region" "current" {}
