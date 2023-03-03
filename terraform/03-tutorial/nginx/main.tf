# This is a module that defines:
# * docker image: nginx
# * docker image: nginx name=<container_name>, running at <nginx_port>

resource "docker_image" "nginx" {
    name = "nginx:latest"
    # keep_locally = false
}

resource "docker_container" "nginx" {
    image = docker_image.nginx.image_id
    name = var.container_name  # input
    ports {
        internal = 80
        external = var.nginx_port  # input
    }
}
