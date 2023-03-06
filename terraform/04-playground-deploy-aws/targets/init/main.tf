# This module will INITIALIZE the workflow:
# it will create an S3 bucket to store youre remote state



# Create an S3 bucket to store remote state
resource "aws_s3_bucket" "terraform_state" {
  bucket_prefix = "tfstate-"

  # Prevent accidental removal
  lifecycle {
    prevent_destroy = true
  }

  tags = { Name = "Terraform State" }
}

# Configure versioning
resource "aws_s3_bucket_versioning" "terraform_state" {
    bucket = aws_s3_bucket.terraform_state.id

    versioning_configuration {
      status = "Enabled"
    }
}
