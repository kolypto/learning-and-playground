# `terraform`: terraform settings
terraform {
  # Providers we need
  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.16"
    }
  }

  # tf version
  required_version = ">= 1.3.0"
}


# `provider`: provider configuration
provider "aws" {
  region = "eu-central-1"
}



# Fetch data about the current region
data "aws_region" "current" { }  # -> data.aws_region.current.name


# `resource`: components of the infrastructure

# Format: resource <type> <name>
# Resulting ID: "aws_instance.app_server"
resource "aws_instance" "app_server" {
  ami           = var.aws_ami  # reference a variable
  instance_type = "t2.micro"

  associate_public_ip_address = true
  # subnet_id = ...
  # vpc_security_group_ids = [aws_security_group.web-sg.id]

  # Amazon supports a user-provided script to set the server up.
  # Use some bash script, with apt-get.
  # Interpolate ${department}
  # user_data = templatefile("user_data.tftpl", { department = var.user_department, name = var.user_name })

  # SSH key
  key_name = aws_key_pair.ssh_key.key_name

  tags = {
    Name = "ExampleAppServerInstance"
  }
}


# Create an S3 bucket
resource "aws_s3_bucket" "sample" {
  bucket = "public-files"

  acl    = "public-read"

  tags = {
    public_bucket = true
  }
}


# AWS key
resource "aws_key_pair" "ssh_key" {
  key_name = "ssh_key"
  public_key = file("ssh_key.pub")  # read from file
}

