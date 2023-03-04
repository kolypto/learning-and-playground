
# Server's public IP. You can SSH into it.
output "server_public_ip" {
    description = "IP address of the server"
    value       = aws_eip.server_ip.public_ip
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
