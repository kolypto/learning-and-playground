# This module will INITIALIZE the workflow:
# it will create an S3 bucket to store youre remote state



# NOTE and TODO:
# Because Terraform stores outputs into state, we may actually use this module as an INTERACTIVE MODULE
# that asks all the parameters from you ONCE, and then we just use its state as variable storage.
# This may be fun, but a `*.tfvars` file is definitely easier to support :) So we don't.
#
# However, some, just some, parameters may be stored here alright.



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
