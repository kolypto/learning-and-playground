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
