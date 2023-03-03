output "container_id" {
    value = docker_container.nginx.name
}

output "service_hostport" {
    value = "${docker_container.nginx.ports[0].ip}:${docker_container.nginx.ports[0].external}"
}