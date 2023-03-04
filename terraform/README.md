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
  providers = {}  # pass provider configurations. By default, a module inherits them.
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
--privileged=true -v /var/run/docker.sock:/var/run/docker.sock
-v ~/.docker/config.json:/root/.docker/config.json:ro
-v ~/.terraformrc:/root/.terraformrc:ro
-v ~/.ssh:/root/.ssh:ro
-v $PWD/.env:/root/.env:ro
```














# === More Examples ===












# 01-docker-tutorial


# 01-docker-tutorial/main.tf

```terraform
terraform {
  # Terraform version
  required_version = "~> 1.3"

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



# 03-tutorial/terraform.tfvars

```terraform
# Variables can be set with this file.
# NOTE: do not commit it!

nginx_port = 8000

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
terraform {
  required_version = "~> 1.3.9"

  required_providers {
    aws = {
      source  = "hashicorp/aws"
      version = "~> 4.57"
    }
  }
}

provider "aws" {
  # The region to use
  region = "eu-central-1"

  # Access key can be provided here
  # access_key = "my-access-key"
  # secret_key = "my-secret-key"

  # The provider can use credentials from ~/.aws/credentials and ~/.aws/config:
  # profile = "default"  # default profile name (from the credentials file)

  # Environment config:
  # $ export AWS_REGION="us-west-2"
  # $ export AWS_ACCESS_KEY_ID="anaccesskey"
  # $ export AWS_SECRET_ACCESS_KEY="asecretkey"

  # Environment, use config file:
  # $ export AWS_CONFIG_FILE=~/.aws/config
  # $ export AWS_SHARED_CREDENTIALS_FILE=~/.aws/credentials
  # $ export AWS_PROFILE="default"

  # If provided with a role ARN, assume this role
  # See blocks: `assume_role`, `assume_role_with_web_identity`,
}

# Data source: current AWS region
data "aws_region" "current" {}

```



# 04-playground-deploy-aws/main.tf

```terraform

# === Infrastructure first === #

# Create the server and its network
module "server" {
    source = "./server-with-network"

    # NOTE: we do not need to initialize providers within a module:
    # because providers from the root module propagate into other modules!

    project_name = "playground"
    server_name = "playground"
}

# Create a database
module "db_postgres" {
    source = "./server-rds-postgres"

    # Put it into the same subnets the server is in
    # NOTE: AWS requires that an RDS instance is in at least 2 availability zone subnets!
    vpc_id = module.server.vpc_id
    subnet_ids = module.server.vpc_server_subnet_ids
}

# === Now deploy the app to this infrastructure === #

# Set up the database
module "app-setup-database" {
    source = "./app-setup-database"

    # Manage this instance
    postgres_url = module.db_postgres.psql_internal_url
    project_name = "playground"
    applications = ["goserver"]
}

```



# 04-playground-deploy-aws/outputs.tf

```terraform

output "server_public_ip" {
    description = "Server IP address. You can SSH into it."
    value = module.server.server_public_ip
}



output "debug" {
    description = "Debug information. Show it: $ terraform output debug"
    value = {
        db = {
            psql_root = module.app-setup-database.psql_root
            psql_applications = module.app-setup-database.psql_applications
        }
        ssh = {
            server = "ssh ${module.server.server_ssh_user}@${module.server.server_public_ip}"
        }
    }
    sensitive = true
}
```





# 04-playground-deploy-aws/server-with-network


# 04-playground-deploy-aws/server-with-network/main.tf

```terraform
# This module will create:
# * AWS instance
# * Network: VPC + subbet
# * SecurityGroup



# "aws_instance": provides a EC2 instance resource
resource "aws_instance" "server" {
    # AMI image to run
    # ami = "ami-0c0933ae5caf0f5f9"  # Hardcoded image id
    ami = data.aws_ami.linux.id  # pick the most recent image from the data source

    # Intance type
    # t2: nano 0.5G, micro 1G, small 2G, medium 4G, large 8G
    # t3a: 2x more expensive, have 2 vCPU, better network performance
    # Use "A1" or "T4g" for ARM instances
    instance_type = "t2.micro"

    # Use a specific availability zone
    availability_zone = "eu-central-1a"  # NOTE: our network interface must also be configured in there!

    # Tags to assign: i.e. the "Name" of the instance.
    # Yes. Name is a tag.
    tags = { Name = var.server_name }

    # CPU credits: "standard" or "unlimited".
    # T2 instances are launched as "standard" by default
    # T3 instances are launched as "unlimited" by default:
    #   a burstable performance instance can sustain high CPU utilization for any period of time.
    credit_specification { cpu_credits = "standard" }

    # Give it access to a network.
    # The network has an IP list a security group associated (~ firewall)
    network_interface {
        # The "server_ips" provides some IP addresses within a VPC.
        # There may be multiple addresses: so we pick the first one: #0
        network_interface_id = aws_network_interface.server_ips.id
        device_index         = 0  # from the ip list
    }

    # Easy way to get a public IP address
    # associate_public_ip_address = true

    # SSH Key Pair to use with this server.
    # See "aws_key_pair" resource
    # Use data source:
    #   key_name = data.aws_key_pair.aws_ssh.key_name
    # or create one:
    key_name = aws_key_pair.ssh_key.key_name

    # Use `user_data` script to initialize the instance
    # user_data = templatefile("user_data.tftpl", { username = var.user_name })
    user_data_replace_on_change = true
    user_data = <<-EOF
        #!/bin/bash
        sudo apt-get update -yq
        sudo apt-get install -yq --no-install-recommends docker.io
    EOF

    # Remote command: i.e. on the server instance
    # provisioner "remote-exec" {
    #     # Run remotely
    #     inline = [
    #         "sudo adduser --disabled-password kolypto",
    #         "sudo apt-get update -yq",
    #         "sudo apt-get install -yq --no-install-recommends docker.io"
    #     ]
    #     connection {
    #         host        = self.public_ip
    #         type        = "ssh"
    #         user        = "ec2-user"
    #         private_key = file(var.ssh_private_key_file)
    #     }
    # }

}


# Give it a public IP address
resource "aws_eip" "server_ip" {
    instance = aws_instance.server.id
    vpc      = true
    # NOTE: you can associate it with a `network_interface` instead of an `instance`.
    # network_interface = aws_network_interface.server_ips.id

    # NOTE: "aws_eip" may require an IGW to exist prior to association!
    # Declare it explicitly:
    depends_on = [ aws_internet_gateway.gw ]
}




# SSH key to access the server with
resource "aws_key_pair" "ssh_key" {
  # Use `key_name` for a static unique name, use `key_name_prefix` for a generated unique name
  key_name_prefix = "${var.server_name}-ssh-key-"
  public_key = file(var.ssh_public_key_file)  # read from file
}






# data."aws_ami": find the most recent Amazon Linux image
data "aws_ami" "linux" {
    # When multiple images are found, take the most recent one.
    # Careful, be sure not to end up with a daily image!
    most_recent = true

    # Only include images from Amazon
    owners = ["amazon"]  # Amazon

    # See also: `name_regex`
    filter {
        name   = "name"

        # values = ["ubuntu/images/hvm-ssd/ubuntu-jammy-22.04-amd64-server-*"]  # Ubuntu Image. User: "ubuntu"
        # values = ["amzn2-ami-kernel-*-hvm-*-x86_64-gp2"]  # Amazon Image. User: "ec2-user"
        values = ["debian-11-amd64-2023*-*"]  # Debian Image. User: "admin"
    }
}

```



# 04-playground-deploy-aws/server-with-network/availability-zones.tf

```terraform

# We want to generate a subnet for every availability zone.
#
# Availability zones can be listed using `data.aws_availability_zones.available`:
#   ["eu-central-1a", "eu-central-1b", "eu-central-1c"]
#
# Let's generate an object

locals {
    # Availability zones:
    # [ { id: 0, char: "a", name: "eu_central-1a"}, ... ]
    availability_zones = [
        for i, az_name in sort(data.aws_availability_zones.available.names) :
            {
                id: i,  # index: 0, 1, ...
                char: substr("abcdefgh", i, 1), # char: "a", "b", ...
                name: az_name,  # az name: "eu_central-1a", ...
            }
    ]
}


# List all availability zones in the current region
data "aws_availability_zones" "available" {
  state = "available"
}

```



# 04-playground-deploy-aws/server-with-network/network.tf

```terraform
# "aws_network_interface": a network interaface that an Instance can use
# A ENI (Elastic Network Interface) is defined as a network device (~ an IP address) in a subnet of a VPC.
resource "aws_network_interface" "server_ips" {
    # Use one subnet: availability zone "a"
    subnet_id   = aws_subnet.server_vpc_subnets["a"].id   # NOTE!!! hardcoded primary subnet :)

    # Give it an IP address inside this network.
    # Note: this list is unordered!
    private_ips = [
        # it may be hardcoded:
        #   "172.16.0.10",
        # but let's calculate a valid IP address using CIDR block:
        cidrhost(aws_subnet.server_vpc_subnets["a"].cidr_block, 10),   # NOTE!!! hardcoded primary subnet :)
    ]

    # Security groups for the interface.
    # It's a sort of a firewall: decides which ports can be open
    security_groups = [
        aws_security_group.server.id,
    ]

    # Name
    tags = { Name = "${var.server_name}-primary-network" }
    description = "Internal network for the server and its services"
}








# "aws_vpc": VPC: Logically Isolated Virtual Private Cloud. A virtual network.
resource "aws_vpc" "server_vpc" {
    # Network: IP range
    cidr_block = "172.16.0.0/16"

    # Defaults:
    enable_dns_support = true  # Enabled DNS support in the VPC. Default: true
    enable_dns_hostnames = true # Enabled DNS hostnames. Default: false

    # Name
    tags = { Name = "${var.project_name} VPC" }
}

# "aws_subnet": a subnet within a VPC
# We actually generate multiple subnets: one for each availability zone.
# So aws_subnet.server_vpc_subnets["a"] is the primary one, in the first availability zone
resource "aws_subnet" "server_vpc_subnets" {
    # Within this VPC
    vpc_id            = aws_vpc.server_vpc.id

    # Let's create a subnet for every availability zone: primary, and secondary
    cidr_block        = each.value.cidr_block
    availability_zone = each.value.availability_zone
    for_each = {
        # We could have hardcoded them:
        #   "a" : { cidr_block = "172.16.0.0/24", availability_zone = "eu-central-1a"},
        #   "b" : { cidr_block = "172.16.1.0/24", availability_zone = "eu-central-1b"},
        # But let's generate:
        for az in local.availability_zones:
            az.char => {
                cidr_block = cidrsubnet("172.16.0.0/16", 8, az.id),  # automatic calculation
                availability_zone = az.name,  # availability zone name
            }
    }

    # Name
    tags = { Name = "${var.project_name}-net-${each.key}" }
}

```



# 04-playground-deploy-aws/server-with-network/network-gateway.tf

```terraform
# If an instance needs a public IP, the VPC must contain a public gateway

# "aws_internet_gateway": provides a VPC access to the Internet
# See also: "aws_egress_only_internet_gateway"
resource "aws_internet_gateway" "gw" {
    vpc_id = aws_vpc.server_vpc.id
    tags = { Name = "${var.project_name} gateway" }
}

# "aws_route": creates a routing entry in a VPC routing table
# See also: "aws_route_table" to have multiple inline routes
resource "aws_route" "gw_route" {
    route_table_id         = aws_vpc.server_vpc.main_route_table_id
    gateway_id             = aws_internet_gateway.gw.id
    destination_cidr_block = "0.0.0.0/0"

    # It can be used to give access to another VPC
    # vpc_peering_connection_id = "pcx-45ff3dc1"
}

```



# 04-playground-deploy-aws/server-with-network/network-securitygroups.tf

```terraform
# NOTE: Amazon had issues with "aws_security_group", and it's now DEPRECATED ⚠️
# All new setups should use "aws_vpc_security_group_egress_rule" and "aws_vpc_security_group_ingress_rule"
# See https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/security_group


# We have just one server, so it does not say "server-api" or "server-frontend"
resource "aws_security_group" "server" {
    # Name prefix: use it to make sure names stay unique
    name_prefix   = "${var.server_name}-server-security-"

    # VPC to define it on
    vpc_id = aws_vpc.server_vpc.id

    # Name
    tags = { Name = "${var.server_name} server security" }
    description = "Allows HTTP and SSH in"
}


# Define inbound / outbound rules, allows certain ports only

resource "aws_vpc_security_group_egress_rule" "server_any_out" {
    security_group_id = aws_security_group.server.id

    description = "Any outbound traffic is ok"
    cidr_ipv4   = "0.0.0.0/0"
    ip_protocol = "-1"  # all protocols: TCP and UDP
    # Use from_port=0 to_port=0 to allow all ports.
    # AWS, however, insists that the values should be "-1"
    from_port   = -1
    to_port     = -1
}


resource "aws_vpc_security_group_ingress_rule" "server_in_http" {
    security_group_id = aws_security_group.server.id

    description = "HTTP in"
    cidr_ipv4   = "0.0.0.0/0"  # any network
    ip_protocol = "tcp"
    # Port range: from (value) to (value)
    from_port   = 80
    to_port     = 80
}

resource "aws_vpc_security_group_ingress_rule" "server_in_ssh" {
    security_group_id = aws_security_group.server.id

    description = "SSH in"
    cidr_ipv4   = "0.0.0.0/0"
    ip_protocol = "tcp"
    from_port   = 22
    to_port     = 22
}

```



# 04-playground-deploy-aws/server-with-network/outputs.tf

```terraform

# Server's public IP. You can SSH into it.
output "server_public_ip" {
    description = "IP address of the server"
    value       = aws_eip.server_ip.public_ip
}


# Server's root user name
output "server_ssh_user" {
    description = "Server's SSH user"
    value = "admin"  # hardcoded for Debian. See precondition below


    precondition {
        condition = startswith(data.aws_ami.linux.name, "debian")
        error_message = <<-EOF
            TODO: at the moment we only know the root user for Debian systems :)
            Replace this hardcode when other systems are in use
        EOF
    }
}


# VPC id. Other resources may be created there.
output "vpc_id" {
    description = "VPC id the server is created in"
    value = aws_vpc.server_vpc.id
}

# The subnet the server's in
output "vpc_server_subnet_ids" {
    description = "The subnet id the server's in"
    value = [for subnet in aws_subnet.server_vpc_subnets: subnet.id]
}

```



# 04-playground-deploy-aws/server-with-network/variables.tf

```terraform
# Your public key file.
# You will use it to SSH into the server.
variable "ssh_public_key_file" {
    type        = string
    description = "SSH public key to add to the instance. You will use it to SSH into it."
    default     = "~/.ssh/id_rsa.pub"
}


# Project name
variable "project_name" {
    type = string
    description = "Name of the project. Networks will have it."
}



# Server name
variable "server_name" {
    type = string
    description = "Name of the server. Object names will depend on it"
}

```





# 04-playground-deploy-aws/server-rds-postgres


# 04-playground-deploy-aws/server-rds-postgres/terraform.tf

```terraform
terraform {
    required_providers {
        # Used to generate a random password for Postgres
        random = {
            source = "hashicorp/random"
            version = "~> 3.4"
        }
    }
}

```



# 04-playground-deploy-aws/server-rds-postgres/main.tf

```terraform
# This module will create:
# * Postgres instance in a subnet





# Get a Postgres database.
# See also: "aws_db_instance_automated_backups_replication"
resource "aws_db_instance" "db" {
    engine = "postgres"
    engine_version = "15.2"
    identifier_prefix = "db-"

    # Postgres
    username = "postgres"
    password = random_password.db_password.result
    db_name  = "postgres"  # create a database
    parameter_group_name = aws_db_parameter_group.db_params.name

    # Instance
    instance_class    = "db.t3.micro"
    storage_encrypted = true  # Encypt data on disk. Default: false
    publicly_accessible = true  # Is it publicly accessible? Default: false

    # Network
    db_subnet_group_name   = aws_db_subnet_group.db_subnet.name
    vpc_security_group_ids = [aws_security_group.db.id]

    # Management
    apply_immediately = false  # Do not wait for a maintenance window, apply changes immediately. Default: false
    deletion_protection = false  # The DB cannot be deleted. Default: false
    delete_automated_backups = true  # Delete backups when the DB is deleted. Default: true
    skip_final_snapshot = true  # Do not make a final snapshot before removing it. Default: false
    performance_insights_enabled = true  # Watch performance. Default: false

    # Maintenance window for: backups and upgrades
    maintenance_window = "Mon:01:00-Mon:03:00"  # UTC
    backup_window      = "00:00-00:59"          # UTC
    backup_retention_period = 3  # Keep backups for N days. Default: 0 = disabled
    copy_tags_to_snapshot = true  # Copy all instance Tags to snapshots. Default: false

    # Upgrades during the maintenance window
    auto_minor_version_upgrade = true   # Auto upgrade minor versions. Default: true
    allow_major_version_upgrade = true  # Allow major upgrades. Default: false?

    # Autoscaling of the hard drive
    allocated_storage     = 10   # Gb of disk space
    max_allocated_storage = 100  # Auto-scale disk space up to this many Gb
}



# Generate a random password.
# Once generated, it will remain.
resource "random_password" "db_password" {
    length = 16
    special = false
}


# Parameters
resource "aws_db_parameter_group" "db_params" {
    name_prefix = "db-params-postgres-"
    family = "postgres15"

    # Parameters: https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/Appendix.PostgreSQL.CommonDBATasks.Parameters.html
    parameter {
        name  = "log_connections"
        value = "1"
    }
}




# NOTE: Terraform may PLAN a change for the next maintenance window!

# NOTE: See RDS Blue/Green deployments for low downtime updates: (only for MySQL/MariaDB)
#   https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/blue-green-deployments.html

```



# 04-playground-deploy-aws/server-rds-postgres/network.tf

```terraform
# Create Postgres in the following subnet
# RDS instance will be created in the VPC this subnet belongs to
resource "aws_db_subnet_group" "db_subnet" {
  name_prefix = "db-subnet-"

  # List of VPC subnet ids to make the DB available in
  # Example: subnet_ids = [aws_subnet.frontend.id, aws_subnet.backend.id]
  subnet_ids = var.subnet_ids

  tags = { Name = "DB Subnet" }
}


# Configure security groups for Postgres
resource "aws_security_group" "db" {
    # Name prefix: use it to make sure names stay unique
    name_prefix   = "db-"

    # VPC to define it on
    vpc_id = var.vpc_id

    # Name
    tags = { Name = "DB Security" }
    description = "DB security: allow PostgreSQL in, nothing out"
}

# Postgres security group: allow incoming Postgres connections
resource "aws_vpc_security_group_ingress_rule" "db_in_postgres" {
    security_group_id = aws_security_group.db.id

    description = "Postgres in"
    cidr_ipv4   = "0.0.0.0/0"
    ip_protocol = "tcp"
    from_port   = 5432
    to_port     = 5432
}

```



# 04-playground-deploy-aws/server-rds-postgres/outputs.tf

```terraform
output "psql_internal_url" {
    description = "Postgres connection URL, admin user, internal"
    value = format(
        "%s://%s:%s@%s:%s/%s",
        aws_db_instance.db.engine,
        aws_db_instance.db.username, aws_db_instance.db.password,
        aws_db_instance.db.endpoint, aws_db_instance.db.port,
        aws_db_instance.db.db_name
    )
}

output "postgres_db" {
    description = "Postgres database connection details"
    value = {
        engine = aws_db_instance.db.engine,
        username = aws_db_instance.db.username,
        password = sensitive(aws_db_instance.db.password),  # one value is sensitive()
        endpoint = aws_db_instance.db.endpoint,
        port = aws_db_instance.db.port,
        db_name = aws_db_instance.db.db_name,
    }
}
```



# 04-playground-deploy-aws/server-rds-postgres/variables.tf

```terraform
variable "vpc_id" {
    type = string
    description = "VPC to create the database in. Used to configure subnet security groups"
}


variable "subnet_ids" {
    type = list(string)
    description = "Subnet IDs that the Database should be made available in. Must be 2+"
    validation {
      condition = length(var.subnet_ids) >= 2
      error_message = <<-EOF
        AWS limitation: an RDS instance must be in 2 or more different availability zones.
        Please provide at least two subnets in different availability zones
      EOF

      # Here's how the error message looks like:
      # > │ Error: creating RDS DB Subnet Group (db-subnet-2023...): DBSubnetGroupDoesNotCoverEnoughAZs:
      # The DB subnet group doesn't meet Availability Zone (AZ) coverage requirement.
      # Current AZ coverage: eu-central-1a. Add subnets to cover at least 2 AZs.
    }
}

```





# 04-playground-deploy-aws/app-setup-database


# 04-playground-deploy-aws/app-setup-database/terraform.tf

```terraform
terraform {
    required_providers {
        postgresql = {
            source = "cyrilgdn/postgresql"
            version = "~> 1.18"
        }
    }
}

```



# 04-playground-deploy-aws/app-setup-database/main.tf

```terraform
# This module will initialize the database for the app.
# It can connect directly to AWS RDS instances (!)
#
# It will create:
# * A database, named `var.project_name`
# * A user who owns this database, with the same name
# * A root user: `<database>-root`
# * For every application, a separate user with ALL permissions: `<database>-<application>`


# Database for the app
resource "postgresql_database" "app" {
  name = var.project_name  # db name
  owner = postgresql_role.owner.name  # only owner can drop it
}




# Root role: owns the database.
# Only they can make changes
resource "postgresql_role" "owner" {
  name = var.project_name
}

# Root user. Only they can make changes to the schema: e.g. migrations
resource "postgresql_role" "root_user" {
  name = "${var.project_name}-root"
  password = "${var.project_name}-root"
  roles = [postgresql_role.owner.name]
  login = true
}




# Application user: the application will use it to make queries.
# Separate user for every app is convenient in terms of logging & monitoring
resource "postgresql_role" "app_users" {
  name = "${var.project_name}-${each.value}"
  password = "${var.project_name}-${each.value}"  # TODO: perhaps a better password?
  login = true

  # Generate a user for every app
  for_each = toset(var.applications[*])
}

# Grant ALL privileges on this database
resource "postgresql_grant" "app_user_grants" {
  role = each.value
  object_type = "database"
  database = postgresql_database.app.name
  privileges = ["ALL"]

  # Generate a grant for every user
  for_each = toset([for user in postgresql_role.app_users: user.name])

  # Postgres provider doesn't like `privileges = ALL`:
  # every time it things that it changed to "CONNECT", "CREATE", "TEMPORARY"
  # Let's ignore it. Because it's already "ALL": can't get any bigger that this.
  lifecycle {
    ignore_changes = [privileges]  # Ignore changes to this attribute
  }
}







# Init provider: where to connect to?
provider "postgresql" {
    # use GoCloud to connect to AWS RDS instances (!)
    # Set endpoint value: host = "instance.xxx.region.rds.amazonaws.com"
    scheme   = "awspostgres"

    # In Amazon RDS, we're not a superuser. Set to `false`: otherwise this error comes up:
    # > could not read role password from Postgres as connected user postgres is not a SUPERUSER
    superuser = false

    # Connect to
    host            = local.db_url.host
    port            = local.db_url.port
    username        = local.db_url.username
    password        = local.db_url.password

    # Timeout is small because we're fast :)
    connect_timeout = 15
}




locals {
  # Parse DB URL into an object: username, password, host, port[, database]
  db_url = regex(join("", [
      "(?:postgres|postgresql)?://?",  # postgres://, postgresql:/
      "(?P<username>.+?)", ":(?P<password>.+?)@",  # user:password@
      "(?P<host>.+?)", ":(?P<port>\\d+)", # host:port
      "(?:/(?P<database>.+))?",  # optional: /database
    ]), var.postgres_url)
}

```



# 04-playground-deploy-aws/app-setup-database/outputs.tf

```terraform

# Root user
output "psql_root" {
    description = "Postgres connection URL: connect as root user (to run migrations)"
    value = format(
        "postgresql://%s:%s@%s:%s/%s",
        postgresql_role.root_user.name, postgresql_role.root_user.password,
        local.db_url.host, local.db_url.port,
        postgresql_database.app.name
    )
    sensitive = true
}


# Application users
output "psql_applications" {
    description = "Postgres connection URL: for each of your applications"
    value = {
        for app_name, app_user in postgresql_role.app_users:
            app_name => format(
                "postgresql://%s:%s@%s:%s/%s",
                app_user.name, app_user.password,
                local.db_url.host, local.db_url.port,
                postgresql_database.app.name
            )
    }
    sensitive = true
}

```



# 04-playground-deploy-aws/app-setup-database/variables.tf

```terraform
# DB connection to manage
variable "postgres_url" {
    type = string
    description = "The DB to manage. Postgres connection url: postgres://user:password@host:port/. Provide AWS Instance URL"
}

# Project name. Used as DB name
variable "project_name" {
    type = string
    description = "Name of the project. Will be used as DB name"
}

# Application names.
# Every application gets their own login.
variable "applications" {
    type = list(string)
    description = "List of application names that will use the DB with their own accounts"
}

```

