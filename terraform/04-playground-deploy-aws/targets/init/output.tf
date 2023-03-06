# Path to the S3 bucket used for storing tfstates
output "s3_backend" {
    description = "Terraform tfstate backend storage to use with other targets"
    value = aws_s3_bucket.terraform_state.id
}

