
# We want to generate a subnet for every availability zone.
#
# Availability zones can be listed using `data.aws_availability_zones.available`:
#   ["eu-central-1a", "eu-central-1b", "eu-central-1c"]
#
# Let's generate an object

locals {
    # Availability zones:
    # [ { id: 0, char: "a", name: "eu_central-1a"}, ... ]
    availability_zones = [
        for i, az_name in sort(data.aws_availability_zones.available.names) :
            {
                id: i,  # index: 0, 1, ...
                char: substr("abcdefgh", i, 1), # char: "a", "b", ...
                name: az_name,  # az name: "eu_central-1a", ...
            }
    ]
}


# List all availability zones in the current region
data "aws_availability_zones" "available" {
  state = "available"
}
