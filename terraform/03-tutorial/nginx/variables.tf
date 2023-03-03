variable "container_name" {
    type = string
    description = "nginx container name to assign"
}

variable "nginx_port" {
    type = number
    description = "The port to run nginx on"
}