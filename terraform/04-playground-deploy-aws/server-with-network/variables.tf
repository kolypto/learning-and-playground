# Your public key file
variable "ssh_public_key_file" {
    type        = string
    description = "SSH public key to add to the instance. You will use it to SSH into it."
    default     = "~/.ssh/id_rsa.pub"
}
