# This module will run a Docker container on your server:
# * Pull the image
# * Start the container


# Update from this image
data "docker_registry_image" "app" {
    name = var.docker_image_name
}

# Pull the image
resource "docker_image" "app" {
    name          = data.docker_registry_image.app.name
    pull_triggers = [data.docker_registry_image.app.sha256_digest]
}

# Deploy the container
resource "docker_container" "app" {
    # Image id
    image    = docker_image.app.image_id

    # Container name
    name     = "app"

    # Assume successful only when the container actually runs. Default: true
    # When `false`, then as long as the container exists, Terraform assumes it is successful (?)
    must_run = true

    # Restart policy for the container: "no", "on-failure[:max-retries]", "always", "unless-stopped". Default: "no"
    restart = "on-failure"
    max_retry_count = 3  # how many times to restart

    # Save container logs. Default: false
    logs = true

    # Environment variables
    env = [
        "TZ=Europe/Oslo",
        "DB_URL=${var.app_database_urls.goserver}",
        "MQTT_HOST=broker.hivemq.com:1883",
    ]

    # Labels to assign
    labels {
        label = ""
        value = ""
    }

    # Management
    # TODO: `wait = true` segfaults. Change to `true` when a new version comes out.
    wait = false       # Wait for the container to be in healthy state. Default: false
    wait_timeout = 20  # Time to wait for the container to become healthy
    stop_timeout = 30  # Timeout to stop


    # See Docker features: privileged, capabilities, memory (limit), networks_advanced, healthcheck { command }
    # See Docker features: entrypoint, workingdir, command, env, ports, restart, labels, mounts, volumes, tmpfs
    # See also: `upload` to upload files to the container before it starts
    # See also: `container_logs`


    # Now link with Traefik

    networks_advanced {
        name = module.traefik.traefik_network_name
    }

    # Generate label{} blocks for every Traefik label
    dynamic "labels" {
        for_each = {
            "traefik.enable": "true",
            "traefik.http.routers.api.rule": "PathPrefix(`/api/v1`)",
            "traefik.http.routers.playground.entrypoints": "http",
            # "traefik.http.routers.playground.tls.certresolver": "route53",
            # # By default, Traefik uses the first exposed port of a container.
            # # Use "traefik.http.services.xxx.loadbalancer.server.port" to override this behavior
            # "traefik.http.services.playground.loadbalancer.server.port": "8888",

        }
        content {
            label = labels.key
            value = labels.value
        }
    }
}




# Set up Traefik
module "traefik" {
    source = "./container-traefik"

    traefik_docker_image = "traefik:2.9"  # TODO: check out 3.0
}




# Networks
data "docker_network" "host" {
  name = "host"
}
data "docker_network" "bridge" {
  name = "bridge"
}
