# This module will start Traefik container


# Traefik container
resource "docker_container" "traefik" {
  image      = docker_image.traefik.image_id
  name       = "traefik"

  logs = true
  wait = false  # TODO: `wait = true` segfaults. Change to `true` when a new version comes out.
  must_run = true
  restart = "on-failure"
  max_retry_count = 3

  # HTTP, HTTPS ports
  ports {
    internal = 80
    external = 80
  }
  ports {
    internal = 443
    external = 443
  }
  # MQTT port
  ports {
    internal = 8883
    external = 8883
  }
  # Traefik manager
  ports {
    internal = 8080
    external = 8080
  }

  # command = [
  #   "--log.level=DEBUG",
  #   "--api.insecure=true",
  #   "--providers.docker=true",
  #   "--providers.docker.exposedbydefault=false",
  #   "--entrypoints.http.address=:80",
  #   "--entrypoints.https.address=:443",
  #   # "--entrypoints.web.http.redirections.entrypoint.to=websecure",
  #   # "--entrypoints.web.http.redirections.entrypoint.scheme=https",
  #   "--entrypoints.mqtts.address=:8883",
  #   # "--certificatesresolvers.route53.acme.tlschallenge=true",
  #   # "--certificatesresolvers.route53.acme.email=root@medthings.no",
  #   # "--certificatesresolvers.route53.acme.storage=/config/letsencrypt/acme.json",
  # ]

  # Configure
  upload {
    content = <<-EOF
    logLevel = "DEBUG"
    defaultEntryPoints = ["http", "https"]

    [log]
      level = "DEBUG"

    [entryPoints]
      [entryPoints.http]
      address = ":80"
      [entryPoints.https]
      address = ":443"
      [entryPoints.mqqts]
      address = ":8883"

    [api]
    insecure = true
    dashboard = true

    [providers]
      [docker]
      exposedByDefault = false
    EOF
    file = "/etc/traefik/traefik.toml"
  }

  # Network
  networks_advanced {
    name = docker_network.traefik.name
  }

  # Mount a volume: /config/letsencrypt will contain LetsEncrypt HTTPS certificates
  volumes {
    container_path = "/config"
    volume_name    = docker_volume.traefik_config.name
  }

  # Bind mount the Docker socket: give Traefik access to local Docker
  mounts {
    source    = "/var/run/docker.sock"
    target    = "/var/run/docker.sock"
    type      = "bind"
    read_only = true
  }
}


# Create a network for traefik
resource "docker_network" "traefik" {
  # Network name
  name = "traefik"

  # TODO: ?
  ipam_config {
    gateway = "172.20.0.1"
    subnet  = "172.20.0.0/16"
  }
}


# Pull Traefik image
resource "docker_image" "traefik" {
  name = var.traefik_docker_image
}


# Create a volume for persistent config.
# Letsencrypt certificates will be put here.
resource "docker_volume" "traefik_config" {
  name = "traefik-config"
}
