

# `variable`: Input value. Parameterization.
# Like this:
#   $ terraform apply -var "aws_ami=..."
# Normally goes into: variables.tf
variable "aws_ami" {
  description = "Amazon Linux"
  type        = string
  default     = "ami-0c0933ae5caf0f5f9"
}

