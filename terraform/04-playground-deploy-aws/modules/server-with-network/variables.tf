# Your public key file.
# You will use it to SSH into the server.
variable "ssh_public_key_file" {
    type        = string
    description = "SSH public key to add to the instance. You will use it to SSH into it."
    default     = "~/.ssh/id_rsa.pub"
}


# Project name
variable "project_name" {
    type = string
    description = "Name of the project. Networks will have it."
}



# Server name
variable "server_name" {
    type = string
    description = "Name of the server. Object names will depend on it"
}


# Server: open ports
variable "server_open_ports" {
    type = list(number)
    description = "The list of ports to keep open (via AWS security group rules)"
}
