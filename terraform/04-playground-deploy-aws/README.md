# === AWS Deployed Application ===

A complete application, deployed to AWS:

* VPC network, EC2 instance, RDS instance
* Docker image pull/push, container, Traefik
* Remote state stored in S3 bucket

## Layout

This is how files are organized

* `./modules` are modules for include. Don't use.
* `./targets` is where you run Terraform apply

## Targets

The `./targets` folder contains modules that you can `apply` in.

### Init

Create the S3 bucket for configuration storage backend:

```console
$ terraform -chdir targets/init init
$ terraform -chdir targets/init apply
```

Init the infrastructure. Give it the name of the bucket you've just created:

## Infrastructure

The Infrastructure module deploys networks and servers, and stores its local state into S3.
You need to specify the bucket that you've created in the "init" step:
just provide it once to the "apply" command, and your terraform state will remember it.

```console
$ terraform -chdir targets/infrastructure init
$ terraform -chdir targets/infrastructure apply
```

## App

App is separate from infrastructure because this way it's faster, and has fewer inter-dependencies.
For instance, I've found out that `provider "docker" { }` configuration actually requires a working server
to SSH into: so unless the infrastructure is up, the dependency does not really resolve correctly.

So, we have a two-step deploy here: first, the infrastructure, and second, the app.
This app module reads remote state from S3, so give it the same bucket you've already provided.

Deploy or reploy the app:

```console
$ terraform -chdir targets/app init
$ terraform -chdir targets/app apply
```

# Other Notes

See `*.tfvars` files: an easy DRY:

```console
$ terraform -chdir targets/app init -backend-config=../../backend.tfvars
$ terraform -chdir targets/app apply -var-file=../../app.tfvars
```

