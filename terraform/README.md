# Terraform

# Command-Line Interface

Load providers:

```console
$ terraform init
$ terraform init -upgrade
```

Preview, apply:

```console
$ terraform validate
$ terraform plan
$ terraform apply
```

Apply with `-replace` when a resource have become unhealthy: reprovision the resource:

```console
$ terraform apply -replace 'docker_container.nginx[1]'
```

Apply, target one specific resource:

```console
$ terraform plan -target "random_pet.bucket_name"
```

Inspect:

```console
$ terraform refresh

$ terraform state list
$ terraform show docker_image.nginx

$ terraform show
$ terraform show -json |  jq '.values.root_module.resources[] | select(.address=="aws_instance.app_server") | .values.public_ip'

$ terraform output
```

Undo:

```console
$ terraform destroy
```

You can export a plan, and then execute it:

```console
$ terraform plan -out=plan-test
$ terraform show -json plan-test | jq ...
$ terraform apply "plan-test"
```

# Inspect

List all providers and resource types:

```console
$ terraform providers
$ terraform providers schema -json | jq '.provider_schemas[].resource_schemas | keys'
[
  "docker_config",
  "docker_container",
  "docker_image",
  "docker_network",
  "docker_plugin",
  "docker_registry_image",
  "docker_secret",
  "docker_service",
  "docker_tag",
  "docker_volume"
]
```

List all resources:

```console
$ terraform state list
module.nginx-pet[0].docker_image.nginx
module.nginx-pet[1].docker_image.nginx
$ terraform state show module.nginx-pet[0].docker_image.nginx
# module.nginx-pet[0].docker_image.nginx:
resource "docker_image" "nginx" {
    id          = "sha256:904b8cb13b93...:latest"
    image_id    = "sha256:904b8cb13b93..."
    name        = "nginx:latest"
    repo_digest = "nginx@sha256:aa0afebbb3c.."
}
```

Use the console to interactively inspect variables:

```console
$ terraform console
> var.private_subnet_cidr_blocks
tolist([
  "10.0.101.0/24",
  "10.0.102.0/24",
  "10.0.103.0/24",
  "10.0.104.0/24",
  "10.0.105.0/24",
  "10.0.106.0/24",
  "10.0.107.0/24",
  "10.0.108.0/24",
])
```

Use *outputs* to get data from terraform. Don't parse JSON!

# AWS

Set environment variables:

```console
$ export AWS_ACCESS_KEY_ID=
$ export AWS_SECRET_ACCESS_KEY=
```

Here's how to get one:

1. https://console.aws.amazon.com/
2. Account menu (upper right)
3. Security Credentials
4. Create Access Key => CLI =>

With Docker, you can create an `.env` file:

```env
AWS_ACCESS_KEY_ID=AKIA...
AWS_SECRET_ACCESS_KEY=...
```

and use it:

```
--env-file=aws.env
```








# Language

## Syntax

Syntax:

```terraform
<block> <label> {
  # arguments
  identifier = expression
}
```

There also is a JSON-based variant of the language!
Use `*.tf.json`

## Files

Every folder is a module.
It has inputs (variables) and outputs.

Terraform always runs in the context of a single *root module*.

Normally, files cannot redefine objects.
However, `override.tf` and `*_override.tf` files can.
They are put on top of the configuration, in lexographical order.

## Meta-Arguments
Every resource supports it:

```terraform
resource "aws_instance" {
  # For dependencies that Terraform cannot automatically infer
  depends_on = [aws_iam_role_policy.example]

  # How many copies to make
  # Use when: instances are identical. Otherwise use for_each
  count = 2

  # Named copies
  for_each = {
    a_group = "eastus"
    another_group = "westus2"
  }
  for_each = toset(["assets", "media"])
  name     = each.key
  location = each.value
  # Use output "vpc_ids" { value = { for k, v in aws_vpc.example : k => v.id } }

  # Specific provider to use, especially when having aliases
  provider = google.europe

  # Special behavior
  lifecycle {
    create_before_destroy = true  # create a new object first, then destroy the old one
    prevent_destroy = true  # never destroy the object (e.g. database)
    ignore_changes = [attribute, ...]  # ignore attributes, e.g. if modified outside of terraform
    replace_triggered_by = [attribute, ...]  # replace the resource when any of the attributes change

    precondition {
      # Assumptions and guarantees: validation.
      # E.g. database URL must be set, etc
      condition = data.aws_ami.example.architecture == "x86_64"
      error_message = "wrong architecture"
    }

    postcondition {
      # ... check something after it's set up
      # Especially useful with: data sources (fetch & check)
    }
  }

}
```

## Timeouts

Some special blocks support `timeouts`: the time after which a resource is considered to fail:

```terraform
resource "aws_db_instance" "example" {
  timeouts {
    create = "60m"
    delete = "2h"
  }
}
```


## Provisioners
Only use as last resort!! When no terraform resource can help you!

```terraform
resource "aws_instance" "web" {
  # ...

  # Execute local command
  provisioner "local-exec" {
    # Use `self` to refer to our attributes
    command = "echo The server's IP address is ${self.private_ip}"
  }

  # Execute before destroy
  provisioner "local-exec" {
    when    = destroy
    command = "echo 'Destroy-time provisioner'"
    on_failure = continue  # ignore errors
  }

  #
}
```

Most provisioners require access to the remote resource via SSH.
A `connection` describes how to access the remote resource.

When nested within a `resource`, it affects all of that resource's provisioners.
When nested within a `provisioner` block, it overrides any resource-level connection settings.

```terraform
# Copy the file via ssh
provisioner "file" {
  source      = "conf/myapp.conf"
  destination = "/etc/myapp.conf"

  connection {
    type     = "ssh"  # "ssh" (default) or "winrm"
    host     = "${var.host}"
    user     = "root"  # defaultL "root"
    password = "${var.root_password}"
    # private_key = ...  # the contents of an SSH private key
    # agent, agent_identity: use ssh agent

    # See also: bastion_host, bastion_user/password, bastion_private_key
  }
}
```

If you need to run a provisioner without a resource, use `"null_resource"`:

```terraform
resource "aws_instance" "cluster" {
  count = 3
  #...
}

resource "null_resource" "cluster" {
  # Changes to any instance of the cluster requires re-provisioning
  triggers = {
    cluster_instance_ids = "${join(",", aws_instance.cluster.*.id)}"
  }

  # Bootstrap script can run on any instance of the cluster
  # So we just choose the first in this case
  connection {
    host = "${element(aws_instance.cluster.*.public_ip, 0)}"
  }

  provisioner "remote-exec" {
    # Bootstrap script called with private_ip of each node in the cluster
    inline = [
      "bootstrap-cluster.sh ${join(" ", aws_instance.cluster.*.private_ip)}",
    ]
  }
}
```

Provisioner: `file`:

```terraform
# "file": copy files/directories from this machine to the resource
resource "aws_instance" "web" {
  # ...

  # Copies the myapp.conf file to /etc/myapp.conf
  provisioner "file" {
    source      = "conf/myapp.conf"
    destination = "/etc/myapp.conf"
  }

  # Copies the string in content into /tmp/file.log
  provisioner "file" {
    content     = "ami used: ${self.ami}"  # direct content
    destination = "/tmp/file.log"
  }
}
```

Provisioner: `local-exec`:

```terraform
# "local-exec" invokes a local executable after a resource is created
# I.e. on the machine running terraform
resource "aws_instance" "web" {
  # ...

  provisioner "local-exec" {
    # The command to run
    command = "echo ${self.private_ip} >> private_ips.txt"

    working_dir = "."  # working dir
    interpreter = ["perl", "-e"]  # alternative command to append the argument to
    environment = { FOO = "bar" }
    when = destroy
  }
}
```

NOTE: to write to local files, use [hashicorp/local](https://registry.terraform.io/providers/hashicorp/local/latest/docs/resources/file)

Provisioner: `remote-exec`:

```terraform
# Invoke a script on a remote resource after it is created
resource "aws_instance" "web" {
  # ...

  # Establishes connection to be used by all
  # generic remote provisioners (i.e. file/remote-exec)
  connection {
    type     = "ssh"
    user     = "root"
    password = var.root_password
    host     = self.public_ip
  }

  # NOTE: use `provisioner "file"` to copy over a script file

  provisioner "remote-exec" {
    # `inline`: list of command strings to run using #!/bin/bash
    inline = [
      "puppet apply",
      "consul join ${aws_instance.web.private_ip}",
    ]

    # Or: run this script (local path)
    script = "script.sh"

    # Or: run these scripts (local paths)
    scripts = ["one.sh", "two.sh"]
  }
}
```



## Provider Configuration

Two provider instances:

```terraform
# Refer: `aws`
provider "aws" {
  region = "us-east-1"
}

# Refer:`aws.west`.
provider "aws" {
  alias  = "west"
  region = "us-west-2"
}

resource "aws_instance" "foo" {
  provider = aws.west  # refer
}

# Pass on to a module
module "aws_vpc" {
  source = "./aws_vpc"
  providers = {
    aws = aws.west
  }
}
```


## Variables

Declare an input variable:

```terraform
variable "docker_ports" {
  type = list(object({
    internal = number
    external = number
    protocol = string
  }))
  default = [
    {
      internal = 8300
      external = 8300
      protocol = "tcp"
    }
  ]
}
```

Arguments:

```terraform
variable "example" {
  type = number  # variable type
  default = 0  # default value, makes the variable optional
  description = "documentation"
  validation {  # validation rules
    condition = var.example > 0
    error_message = "must be positive"
  }
  sensitive = true  # do not output passwords. Also use: sensitive() func to mark a value
  nullable = false  # Do not allow `null`
}
```

Types:

* `string`, `number`, `bool`
* `list(type)`, `set(type)`,
* `map(type)`, `object(attr = type)`,
* `tuple([type, ...])`
* `any` -- any type

Variable values for the root module are taken from:

* `-var` command line or `-var-file`
* `*.auto.tfvars` files
* `terraform.tfvars` or `terraform.tfvars.json` files
* `TF_VAR_*` environment variables (fallback)


## Modules

Calling child modules:

```terraform
module "consul" {
  source  = "hashicorp/consul/aws"  # local, terraform registry, github, git, http url, s3 bucket, etc
  version = "0.0.5"  # version-constraint

  # Meta-arguments
  count = 3  # how many
  for_each = ...  # multiple instances
  providers = {}  # pass provider configurations
  depends_on = ... # explicit dependency

  # Arguments
  servers = 3
}
```

Refer to their output values:

```terraform
output "something" {
  value = module.consul.instance_ids
}
```


## Expressions

Variable types:

* string, number, bool
* list(string), tuple
* map(string) key/value pairs, object
* null

Literals and strings:

```terraform
resource "something" {
  str = "line \n line \n escaped $${} escaped $${}"

  # Multi-line string.
  heredoc = <<-EOT
    line
    line
    line
  EOT

  # JSON
  example = jsonencode({
    a = 1
    b = "hello"
  })

  # Interpolation
  greeting = "Hello, ${var.name}!"

  # Directives: if, for
  greeting = "Hello, %{ if var.name != "" }${var.name}%{ else }unnamed%{ endif }!"
  servers = <<-EOT
  %{ for ip in aws_instance.example.*.private_ip }
  server ${ip}
  %{ endfor }
  EOT
}
```

### References

Refer to resources:

* `resource_type.name`
* `var.name`
* `local.name`
* `module.name`
* `data.data_type.name`

### Automatic Variables

These values are also available:

* `path.module`: filesystem path of this module
* `path.root`: fs path of the root module
* `path.cwd`: fs path of the original working directory: i.e. before `-chdir` is applied
* `terraform.workspace`: name of the current [workspace](https://developer.hashicorp.com/terraform/language/state/workspaces)

Example use of workspaces:

```terraform
module "example" {
  # ...

  name_prefix = "app-${terraform.workspace}"
}
```

Block-Local values:

* `count.index`: current iteration of the `count` meta-argument
* `each.key`, `each.value`: current item of the `for_each` meta-argument
* `self`: in `provisioner` and `connection` blocks — refers to the current resource

### Operators

Available operators:

* Math: `+`, `-`, `*`, `/`, `%`, `-a`
* Equality: `==`, `!=`
* Comparison: `<`, `<=`, `>=`, `>`
* Logical: `||`, `&&`, `!`

Ternary operator:

```terraform
var.example ? tostring(12) : "hello"
```

### Function Calls

See [Built-in functions](https://developer.hashicorp.com/terraform/language/functions):
numbers, strings, collections, encoding, filesystem, date and time, hash, ip, type conversion

```terraform
# Expand tuples
min([55, 2453, 2]...)
```



### For Expressions

Loops:

```terraform
# Lists
[for s in var.list : upper(s)]
[for i, v in var.list : "#${i} is ${v}"]  # with indexing

# Maps
[for k, v in var.map : length(k) + length(v)]

# Object comprehension:
# Produce an object, not a list
{for s in var.list : s => upper(s)}

# Filtering
[for s in var.list : upper(s) if s != ""]
admin_users = {
  for name, user in var.users : name => user
  if user.is_admin
}

# Use arbitrary ordering (set)
toset([for e in var.set : e.example])

# Grouping: duplicate values won't overwrite. They'll be put into a list.
users_by_role = {
  for name, user in var.users : user.role => name...
}
```

### Splat Expression

Equivalent:

```terraform
# Same result
[for o in var.list : o.id]
var.list[*].id

# Same result
var.list[*].interfaces[0].name
[for o in var.list : o.interfaces[0].name]
```


## Dynamic Blocks

Some nested blocks can be repeated, like `setting`:

```terraform
resource "aws_elastic_beanstalk_environment" "tfenvtest" {
  name                = "tf-test-name"
  application         = "${aws_elastic_beanstalk_application.tftest.name}"
  solution_stack_name = "64bit Amazon Linux 2018.03 v2.11.4 running Go 1.12.6"

  # Produce multiple "setting" blocks
  # dynamic "block_name" { ... content { } }
  dynamic "setting" {
    for_each = var.settings
    #iterator = "setting"  # rename the iterator variable
    #labels =

    content {
      namespace = setting.value["namespace"]
      name = setting.value["name"]
      value = setting.value["value"]
    }
  }
}
```



## Validation and Preconditions

Variable with validation:

```terraform
variable "image_id" {
  type        = string
  description = "The id of the machine image (AMI) to use for the server."

  validation {
    condition     = length(var.image_id) > 4 && substr(var.image_id, 0, 4) == "ami-"
    error_message = "The image_id value must be a valid AMI id, starting with \"ami-\"."
  }
}
```

Resource with a precondition/postcondition:

```terraform
data "aws_ami" "example" {
  id = var.aws_ami_id

  lifecycle {
    # The AMI ID must refer to an existing AMI that has the tag "nomad-server".
    postcondition {
      condition     = self.tags["Component"] == "nomad-server"
      error_message = "tags[\"Component\"] must be \"nomad-server\"."
    }
  }
}
```








# Recipes

## Use Yandex mirror in Russia

Put this into `~/.terraformrc` to use a different mirror:

```terraform
provider_installation {
  network_mirror {
    url = "https://terraform-mirror.yandexcloud.net/"
    include = ["registry.terraform.io/*/*"]
  }
  direct {
    exclude = ["registry.terraform.io/*/*"]
  }
}
```


## Use with Docker (without local setup)

You don't have to set it up.
Let's use Docker:

```console
$ docker run --rm -it -v $PWD:/ops -w /ops/ hashicorp/terraform -chdir=folder/ci init
```

Additional features you may want to add:

```
--env-file=.env
-v ~/.terraformrc:/root/.terraformrc
--privileged=true -v /var/run/docker.sock:/var/run/docker.sock
-v ~/.docker/config.json:/root/.docker/config.json
```














# === More Examples ===












# 01-docker-tutorial


# 01-docker-tutorial/main.tf

```terraform
terraform {
  # Terraform version
  required_version = ">= 0.13"

  # Provider: use Docker
  required_providers {
    docker = {
      source  = "kreuzwerker/docker"
      version = "~> 3.0.1"
    }
  }
}

# Configure provider
provider "docker" {}

# Docker image
resource "docker_image" "nginx" {
  name         = "nginx:latest"
  keep_locally = false
}

# Docker container
# Use `terraform show` to fin all the values
resource "docker_container" "nginx" {
  image = docker_image.nginx.image_id
  name  = "tutorial"
  ports {
    internal = 80
    external = 8000
  }

}


# Now:
# $ terraform plan
# $ terraform apply
# $ terraform destroy

```





# 02-aws-tutorial


# 02-aws-tutorial/main.tf

```terraform
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


```



# 02-aws-tutorial/outputs.tf

```terraform

# `output`: get values from the configuration
# Filename: outputs.tf
output "instance_id" {
  description = "ID of the EC2 instance"
  value       = aws_instance.app_server.id
}

output "instance_public_ip" {
  description = "Public IP address of the EC2 instance"
  value       = aws_instance.app_server.public_ip
}

```



# 02-aws-tutorial/variables.tf

```terraform


# `variable`: Input value. Parameterization.
# Like this:
#   $ terraform apply -var "aws_ami=..."
# Normally goes into: variables.tf
variable "aws_ami" {
  description = "Amazon Linux"
  type        = string
  default     = "ami-0c0933ae5caf0f5f9"
}


```





# 03-tutorial


# 03-tutorial/locals.tf

```terraform
# Unlike variables, locals do not change their value
# Unlike variables, locels can use dynamic expressions and resource arguments!
locals {
    # Use it: "${local.container_name}"
    container_name = "hello-${random_pet.dog.id}"
}
```



# 03-tutorial/main.tf

```terraform
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

```





# 03-tutorial/nginx


# 03-tutorial/nginx/main.tf

```terraform
# This is a module that defines:
# * docker image: nginx
# * docker image: nginx name=<container_name>, running at <nginx_port>

resource "docker_image" "nginx" {
    name = "nginx:latest"
    # keep_locally = false
}

resource "docker_container" "nginx" {
    image = docker_image.nginx.image_id
    name = var.container_name  # input
    ports {
        internal = 80
        external = var.nginx_port  # input
    }
}

```



# 03-tutorial/nginx/outputs.tf

```terraform
output "container_id" {
    value = docker_container.nginx.name
}

output "service_hostport" {
    value = "${docker_container.nginx.ports[0].ip}:${docker_container.nginx.ports[0].external}"
}
```



# 03-tutorial/nginx/variables.tf

```terraform
variable "container_name" {
    type = string
    description = "nginx container name to assign"
}

variable "nginx_port" {
    type = number
    description = "The port to run nginx on"
}
```



# 03-tutorial/nginx/versions.tf

```terraform
terraform {
  required_providers {
    docker = {
      source = "kreuzwerker/docker"
      version = "~> 3.0.1"
    }
  }
}

```





# 03-tutorial


# 03-tutorial/outputs.tf

```terraform
# Example output for multiple containers

# Length of an array
output "nginx_hosts_count" {
  value = length(module.nginx-pet)

  # Validation
  precondition {
    condition     = var.structured.terraform_managed
    error_message = "The server's root volume is not encrypted."
  }
}

# Map: { container_id => host:port }
output "nginx_hosts" {
  description = "nginx host names: list of objects: { name: 'container-name', host: 'ip:port' }"
  # how to access outputs: `module.<module-name>.<output-name>`
  value = [
    for container in module.nginx-pet[*] :
        { name : container.container_id,
          host : container.service_hostport,
        }
    ]
  # Hide passwords
  sensitive   = false
}

# Container names, list
output "nginx_container_names" {
  value = module.nginx-pet[*].container_id
}

```



# 03-tutorial/variables.tf

```terraform
# How to set variables:
# * Using module { ... }
# * Using apply -var or -var-file
# * Using terraform.tfvars or *.auto.tfvars
# * Using TF_VAR_* environment

variable "nginx_port" {
  type = number
  description = "The port to run nginx on"

  # Validation rules
  validation {
    condition = var.nginx_port >=0 && var.nginx_port <= 65535
    error_message = "Port number 0..65535"
  }

  # Sensitive value: do not print
  sensitive = false
}


# Structured variable
variable "structured" {
  # With maps: use lookup() to translate keys (e.g. AWS region) to values (e.g. region-local AMI)
  type = object({
    terraform_managed     = bool
    error_document_key    = optional(string, "error.html")
    index_document_suffix = optional(string, "index.html")
    www_path              = optional(string)
  })
  default = {terraform_managed=false}
}

variable "cors_rules" {
  description = "List of CORS rules."
  type = list(object({
    allowed_headers = optional(set(string)),
    allowed_methods = set(string),
    allowed_origins = set(string),
    expose_headers  = optional(set(string)),
    max_age_seconds = optional(number)
  }))
  default = []
}





# Other variable types:
# * string, number, bool
# * list(string), tuple
# * map(string) key/value pairs, object
# * null


# Use:
# * var.name reference
# * "${var.name}" in strings

```



# 03-tutorial/versions.tf

```terraform
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

```





# 04-playground-deploy-aws


# 04-playground-deploy-aws/terraform.tf

```terraform

```





# examples


# examples/example-ec2.tf

```terraform
terraform {
  /* Uncomment this block to use Terraform Cloud for this tutorial
  cloud {
    organization = "organization-name"
    workspaces {
      name = "learn-terraform-module-use"
    }
  }
  */

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.49.0"
    }
  }
  required_version = ">= 1.1.0"
}




provider "aws" {
  region = "us-west-2"

  default_tags {
    tags = {
      hashicorp-learn = "module-use"
    }
  }
}

module "vpc" {
  source  = "terraform-aws-modules/vpc/aws"
  version = "3.18.1"

  name = var.vpc_name
  cidr = var.vpc_cidr

  azs             = var.vpc_azs
  private_subnets = var.vpc_private_subnets
  public_subnets  = var.vpc_public_subnets

  enable_nat_gateway = var.vpc_enable_nat_gateway

  tags = var.vpc_tags
}

module "ec2_instances" {
  source  = "terraform-aws-modules/ec2-instance/aws"
  version = "4.3.0"

  count = 2
  name  = "my-ec2-cluster-${count.index}"

  ami                    = "ami-0c5204531f799e0c6"
  instance_type          = "t2.micro"
  vpc_security_group_ids = [module.vpc.default_security_group_id]
  subnet_id              = module.vpc.public_subnets[0]

  tags = {
    Terraform   = "true"
    Environment = "dev"
  }
}











variable "vpc_name" {
  description = "Name of VPC"
  type        = string
  default     = "example-vpc"
}

variable "vpc_cidr" {
  description = "CIDR block for VPC"
  type        = string
  default     = "10.0.0.0/16"
}

variable "vpc_azs" {
  description = "Availability zones for VPC"
  type        = list(string)
  default     = ["us-west-2a", "us-west-2b", "us-west-2c"]
}

variable "vpc_private_subnets" {
  description = "Private subnets for VPC"
  type        = list(string)
  default     = ["10.0.1.0/24", "10.0.2.0/24"]
}

variable "vpc_public_subnets" {
  description = "Public subnets for VPC"
  type        = list(string)
  default     = ["10.0.101.0/24", "10.0.102.0/24"]
}

variable "vpc_enable_nat_gateway" {
  description = "Enable NAT gateway for VPC"
  type        = bool
  default     = true
}

variable "vpc_tags" {
  description = "Tags to apply to resources created by VPC module"
  type        = map(string)
  default = {
    Terraform   = "true"
    Environment = "dev"
  }
}


output "vpc_public_subnets" {
  description = "IDs of the VPC's public subnets"
  value       = module.vpc.public_subnets
}

output "ec2_instance_public_ips" {
  description = "Public IP addresses of EC2 instances"
  value       = module.ec2_instances[*].public_ip
}

```



# examples/example-s3.tf

```terraform

# https://developer.hashicorp.com/terraform/tutorials/modules/module-create

resource "aws_s3_bucket" "s3_bucket" {
  bucket = var.bucket_name

  tags = var.tags
}

resource "aws_s3_bucket_website_configuration" "s3_bucket" {
  bucket = aws_s3_bucket.s3_bucket.id

  index_document {
    suffix = "index.html"
  }

  error_document {
    key = "error.html"
  }
}

resource "aws_s3_bucket_acl" "s3_bucket" {
  bucket = aws_s3_bucket.s3_bucket.id

  acl = "public-read"
}

resource "aws_s3_bucket_policy" "s3_bucket" {
  bucket = aws_s3_bucket.s3_bucket.id

  policy = jsonencode({
    Version = "2012-10-17"
    Statement = [
      {
        Sid       = "PublicReadGetObject"
        Effect    = "Allow"
        Principal = "*"
        Action    = "s3:GetObject"
        Resource = [
          aws_s3_bucket.s3_bucket.arn,
          "${aws_s3_bucket.s3_bucket.arn}/*",
        ]
      },
    ]
  })
}





variable "bucket_name" {
  description = "Name of the s3 bucket. Must be unique."
  type        = string
}

variable "tags" {
  description = "Tags to set on the bucket."
  type        = map(string)
  default     = {}
}





output "arn" {
  description = "ARN of the bucket"
  value       = aws_s3_bucket.s3_bucket.arn
}

output "name" {
  description = "Name (id) of the bucket"
  value       = aws_s3_bucket.s3_bucket.id
}

output "domain" {
  description = "Domain name of the bucket"
  value       = aws_s3_bucket_website_configuration.s3_bucket.website_domain
}













```

