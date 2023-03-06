
output "server_public_ip" {
    description = "Server IP address. You can SSH into it."
    value = module.server.server_public_ip
}



# Show it:
#   $ terraform output debug
output "debug" {
    description = "Debug information. Show it: $ terraform output debug"
    value = {
        db = {
            psql_root = module.app_setup_database.psql_root
            psql_applications = module.app_setup_database.psql_applications
        }
        ssh = {
            server = "ssh ${module.server.server_ssh_user}@${module.server.server_public_ip}"
        }
        deployed_image = {
            image_id = module.app_docker_image.deployed_image_id
            name = module.app_docker_image.deployed_image_name
        }
    }
    sensitive = true
}
