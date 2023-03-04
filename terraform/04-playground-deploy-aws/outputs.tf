
output "server_public_ip" {
    description = "Server IP address. You can SSH into it."
    value = module.server.server_public_ip
}



output "debug" {
    description = "Debug information. Show it: $ terraform output debug"
    value = {
        db = {
            psql_root = module.app-setup-database.psql_root
            psql_applications = module.app-setup-database.psql_applications
        }
        ssh = {
            server = "ssh ${module.server.server_ssh_user}@${module.server.server_public_ip}"
        }
    }
    sensitive = true
}