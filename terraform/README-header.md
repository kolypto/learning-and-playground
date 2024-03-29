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

Debug logging:

```console
$ TF_LOG=trace terraform apply ...
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

Get a sensitive value:

```console
$ terraform show -json | jq '.values.root_module.resources[] | select(.address == "tls_private_key.server_ssh_key")'
```

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








