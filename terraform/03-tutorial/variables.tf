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
