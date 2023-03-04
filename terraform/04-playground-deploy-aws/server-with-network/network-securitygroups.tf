# NOTE: Amazon had issues with "aws_security_group", and it's now DEPRECATED ⚠️
# All new setups should use "aws_vpc_security_group_egress_rule" and "aws_vpc_security_group_ingress_rule"
# See https://registry.terraform.io/providers/hashicorp/aws/latest/docs/resources/security_group


# We have just one server, so it does not say "server-api" or "server-frontend"
resource "aws_security_group" "server" {
    # Name prefix: use it to make sure names stay unique
    name_prefix   = "playground-server-security-"

    # VPC to define it on
    vpc_id = aws_vpc.server_vpc.id

    # Name
    tags = { Name = "Playground Server Security" }
    description = "Sever Security that allows HTTP and SSH in"
}


# Define inbound / outbound rules, allows certain ports only

resource "aws_vpc_security_group_egress_rule" "server_any_out" {
    security_group_id = aws_security_group.server.id

    description = "Any outbound traffic is ok"
    cidr_ipv4   = "0.0.0.0/0"
    ip_protocol = "-1"  # all protocols: TCP and UDP
    # Use from_port=0 to_port=0 to allow all ports.
    # AWS, however, insists that the values should be "-1"
    from_port   = -1
    to_port     = -1
}


resource "aws_vpc_security_group_ingress_rule" "server_in_http" {
    security_group_id = aws_security_group.server.id

    description = "HTTP in"
    cidr_ipv4   = "0.0.0.0/0"  # any network
    ip_protocol = "tcp"
    # Port range: from (value) to (value)
    from_port   = 80
    to_port     = 80
}

resource "aws_vpc_security_group_ingress_rule" "server_in_ssh" {
    security_group_id = aws_security_group.server.id

    description = "SSH in"
    cidr_ipv4   = "0.0.0.0/0"
    ip_protocol = "tcp"
    from_port   = 22
    to_port     = 22
}
