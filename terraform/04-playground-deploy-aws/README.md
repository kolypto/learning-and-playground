# AWS Deployed Application

Layout:

* `./modules` are modules for include. Don't use.
* `./targets` is where you run Terraform apply

Targets:

Create the S3 bucket for configuration storage backend:

```console
$ terraform -chdir targets/init init
$ terraform -chdir targets/init apply
```

Init the infrastructure. Give it the name of the bucket you've just created:


```console
$ terraform -chdir targets/infrastructure init
$ terraform -chdir targets/infrastructure apply
```

Deploy or reploy the app:

```console
$ terraform -chdir targets/app init
$ terraform -chdir targets/app apply
```

See `*.tfvars` files: an easy DRY:

```console
$ terraform -chdir targets/app init -backend-config=../../backend.tfvars
$ terraform -chdir targets/app apply -var-file=../../app.tfvars
```

