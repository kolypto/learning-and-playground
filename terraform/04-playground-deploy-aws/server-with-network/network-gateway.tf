# If an instance needs a public IP, the VPC must contain a public gateway

# "aws_internet_gateway": provides a VPC access to the Internet
# See also: "aws_egress_only_internet_gateway"
resource "aws_internet_gateway" "gw" {
    vpc_id = aws_vpc.server_vpc.id
    tags = { Name = "Playground Server Published" }
}

# "aws_route": creates a routing entry in a VPC routing table
# See also: "aws_route_table" to have multiple inline routes
resource "aws_route" "gw_route" {
    route_table_id         = aws_vpc.server_vpc.main_route_table_id
    gateway_id             = aws_internet_gateway.gw.id
    destination_cidr_block = "0.0.0.0/0"

    # It can be used to give access to another VPC
    # vpc_peering_connection_id = "pcx-45ff3dc1"
}
