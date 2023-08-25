# This module will create:
# * AWS instance
# * Network: VPC + subbet
# * SecurityGroup
# * EC2 server with Docker pre-installed



# "aws_instance": provides a EC2 instance resource
resource "aws_instance" "server" {
    # AMI image to run
    # ami = "ami-0c0933ae5caf0f5f9"  # Hardcoded image id
    ami = data.aws_ami.linux.id  # pick the most recent image from the data source

    # Intance type
    # t2: nano 0.5G, micro 1G, small 2G, medium 4G, large 8G
    # t3a: 2x more expensive, have 2 vCPU, better network performance
    # Use "A1" or "T4g" for ARM instances
    instance_type = "t2.micro"

    # Use a specific availability zone
    availability_zone = "eu-central-1a"  # NOTE: our network interface must also be configured in there!

    # Tags to assign: i.e. the "Name" of the instance.
    # Yes. Name is a tag.
    tags = { Name = var.server_name }

    # CPU credits: "standard" or "unlimited".
    # T2 instances are launched as "standard" by default
    # T3 instances are launched as "unlimited" by default:
    #   a burstable performance instance can sustain high CPU utilization for any period of time.
    credit_specification { cpu_credits = "standard" }

    # Give it access to a network.
    # The network has an IP list a security group associated (~ firewall)
    network_interface {
        # The "server_ips" provides some IP addresses within a VPC.
        # There may be multiple addresses: so we pick the first one: #0
        network_interface_id = aws_network_interface.server_ips.id
        device_index         = 0  # from the ip list
    }

    # Disk
    ebs_block_device {
        volume_size = 15  // Gb
        device_name = "/dev/sda1"
    }

    # Easy way to get a public IP address
    # associate_public_ip_address = true

    # SSH Key Pair to use with this server.
    # See "aws_key_pair" resource
    # Use data source:
    #   key_name = data.aws_key_pair.aws_ssh.key_name
    # or create one:
    key_name = aws_key_pair.ssh_key.key_name

    # Use `user_data` script to initialize the instance
    # user_data = templatefile("user_data.tftpl", { username = var.user_name })  # example: template
    # Install Docker
    user_data_replace_on_change = true
    user_data = templatefile("${path.module}/template.server-init.sh", {})

    // OR: Use cloud-config to initialize the instance
    user_data = templatefile("${path.module}/instance-config.template.yml", {
        hostname = local.hostname,
        ssh_admin_pubkey = aws_key_pair.server_ssh_key.public_key,
    })

    # Remote command: i.e. on the server instance
    # provisioner "remote-exec" {
    #     # Run remotely
    #     inline = [
    #         "sudo adduser --disabled-password kolypto",
    #         "sudo apt-get update -yq",
    #         "sudo apt-get install -yq --no-install-recommends docker.io"
    #     ]
    #     connection {
    #         host        = self.public_ip
    #         type        = "ssh"
    #         user        = "ec2-user"
    #         private_key = file(var.ssh_private_key_file)
    #     }
    # }

    # Custom MOTD: users will see this when the SSH
    provisioner "file" {
        destination = "/etc/motd"
        content = <<-EOF
            ########## WARNING WARNING WARNING ##########
                THIS IS A PRODUCTION SERVER
            ########## WARNING WARNING WARNING ##########
        EOF
    }
}


# Give it a public IP address
resource "aws_eip" "server_ip" {
    instance = aws_instance.server.id
    vpc      = true
    # NOTE: you can associate it with a `network_interface` instead of an `instance`.
    # network_interface = aws_network_interface.server_ips.id

    # NOTE: "aws_eip" may require an IGW to exist prior to association!
    # Declare it explicitly:
    depends_on = [ aws_internet_gateway.gw ]
}




# SSH key to access the server with
resource "aws_key_pair" "ssh_key" {
  # Use `key_name` for a static unique name, use `key_name_prefix` for a generated unique name
  key_name = "${var.hostname}-ssh-key"
  public_key = tls_private_key.server_ssh_key.public_key_openssh
  public_key = file(var.ssh_public_key_file)  # OR: read from file
}

# Generate a unique SSH key for this server
resource "tls_private_key" "server_ssh_key" {
  algorithm = "ED25519"
}






# data."aws_ami": find the most recent Amazon Linux image
data "aws_ami" "linux" {
    # When multiple images are found, take the most recent one.
    # Careful, be sure not to end up with a daily image!
    most_recent = true

    # Only include images from Amazon
    owners = ["amazon"]  # Amazon

    # See also: `name_regex`
    filter {
        name   = "name"

        # values = ["ubuntu/images/hvm-ssd/ubuntu-jammy-22.04-amd64-server-*"]  # Ubuntu Image. User: "ubuntu"
        # values = ["amzn2-ami-kernel-*-hvm-*-x86_64-gp2"]  # Amazon Image. User: "ec2-user"
        values = ["debian-11-amd64-2023*-*"]  # Debian Image. User: "admin"
    }
}
