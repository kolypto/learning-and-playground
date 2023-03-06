
# NOTE: if you're tired of entering the same variables every time,
# export them as environment variables:
#
#   TF_VAR_project_name=playground
#   TF_VAR_server_open_ports=[80,443,8080]
#
# Or use -var-file
#   $ terraform -chdir targets/infrastructure apply -var-file=../../playground.tfvars

variable "project_name" {
    description = "Name of the project to use. Lowercase."
    type = string
}

variable "server_open_ports" {
    description = "Ports to keep open on the server. Example: [22, 80, 443]"
    type = list(number)
}
