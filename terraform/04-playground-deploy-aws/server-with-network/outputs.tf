
# Server's public IP. You can SSH into it.
output "server_public_ip" {
    description = "IP address of the server"
    value       = aws_eip.server_ip.public_ip
}


# Server's root user name
output "server_ssh_user" {
    description = "Server's SSH user"
    value = "admin"  # hardcoded for Debian. See precondition below


    precondition {
        condition = startswith(data.aws_ami.linux.name, "debian")
        error_message = <<-EOF
            TODO: at the moment we only know the root user for Debian systems :)
            Replace this hardcode when other systems are in use
        EOF
    }
}


# VPC id. Other resources may be created there.
output "vpc_id" {
    description = "VPC id the server is created in"
    value = aws_vpc.server_vpc.id
}

# The subnet the server's in
output "vpc_server_subnet_ids" {
    description = "The subnet id the server's in"
    value = [for subnet in aws_subnet.server_vpc_subnets: subnet.id]
}
