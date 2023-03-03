# Defines two nginx containers:
# 1. Using `docker_container`
# 2. Using a local module, `nginx`
# Also includes the `hello` module: returns a random pet name
#
# The `versions.tf` contains the `terraform{ }` block with provider versions
#
# The `nginx/` directory contains a module that defines the nginx container.
# It accepts two inputs: "container_name", and "nginx_port"
#


# Run nginx container on port 8001
# Use /nginx/ as a module, provide input variables
module "nginx-pet" {
  # Source: local folder, terraform registry, or github
  source = "./nginx"  # use our tf files in a directory

  # Multiple resources
  count = 2  # number of instances

  # Or by name
  # Provide a map(any) variable and `for_each = var.project` over keys using `each.key` and `each.value`
  # for_each = []

  # Depends on another module
  # depends_on = [module.vpc]

  # Inputs
  container_name = "hello-${random_pet.dog.id}-${count.index}"  # generated + counter
  nginx_port = var.nginx_port + count.index  # input + counter

  # Outputs: -
}


# The "random" resource provides managed randomness: generates a random value on creation, and then holds steady.
# "random_pet": generate fancy "charming-lucy" names, or random ids
resource "random_pet" "dog" {
  # arbitrary key/value pairs that should be selected such that they remain the same until new random values are desired.
#   keepers = {
#     ami_id = var.ami_id
#   }
#  byte_length = 8
  length = 2
}
