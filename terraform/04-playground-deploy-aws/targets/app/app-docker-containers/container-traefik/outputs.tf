output "traefik_network_name" {
    description = "Traefik Docker network name. Use in networks_advanced { name }"
    value = docker_network.traefik.name
}