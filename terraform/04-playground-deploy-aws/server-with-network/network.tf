# "aws_network_interface": a network interaface that an Instance can use
# A ENI (Elastic Network Interface) is defined as a network device (~ an IP address) in a subnet of a VPC.
resource "aws_network_interface" "server_ips" {
    # Use one subnet: availability zone "a"
    subnet_id   = aws_subnet.server_vpc_subnets["a"].id   # NOTE!!! hardcoded primary subnet :)

    # Give it an IP address inside this network.
    # Note: this list is unordered!
    private_ips = [
        # it may be hardcoded:
        #   "172.16.0.10",
        # but let's calculate a valid IP address using CIDR block:
        cidrhost(aws_subnet.server_vpc_subnets["a"].cidr_block, 10),   # NOTE!!! hardcoded primary subnet :)
    ]

    # Security groups for the interface.
    # It's a sort of a firewall: decides which ports can be open
    security_groups = [
        aws_security_group.server.id,
    ]

    # Name
    tags = { Name = "playground-primary-network" }
    description = "Internal network for the server and its services"
}








# "aws_vpc": VPC: Logically Isolated Virtual Private Cloud. A virtual network.
resource "aws_vpc" "server_vpc" {
    # Network: IP range
    cidr_block = "172.16.0.0/16"

    # Defaults:
    enable_dns_support = true  # Enabled DNS support in the VPC. Default: true
    enable_dns_hostnames = true # Enabled DNS hostnames. Default: false

    # Name
    tags = { Name = "Playground Net" }
}

# "aws_subnet": a subnet within a VPC
# We actually generate multiple subnets: one for each availability zone.
# So aws_subnet.server_vpc_subnets["a"] is the primary one, in the first availability zone
resource "aws_subnet" "server_vpc_subnets" {
    # Within this VPC
    vpc_id            = aws_vpc.server_vpc.id

    # Let's create a subnet for every availability zone: primary, and secondary
    cidr_block        = each.value.cidr_block
    availability_zone = each.value.availability_zone
    for_each = {
        # We could have hardcoded them:
        #   "a" : { cidr_block = "172.16.0.0/24", availability_zone = "eu-central-1a"},
        #   "b" : { cidr_block = "172.16.1.0/24", availability_zone = "eu-central-1b"},
        # But let's generate:
        for az in local.availability_zones:
            az.char => {
                cidr_block = cidrsubnet("172.16.0.0/16", 8, az.id),  # automatic calculation
                availability_zone = az.name,  # availability zone name
            }
    }

    # Name
    tags = { Name = "playground-net-${each.key}" }
}
