project_name = "playground"
server_open_ports=[22, 80, 443, 8080]

remote_state_s3_bucket = "tfstate-20230306213054377200000001"
remote_state_infrastructure = "playground/infrastructure"
app_docker_image_ecr_permissions = {
    push_users = [
        "arn:aws:iam::352980582205:user/trygve@medthings.no",
        "arn:aws:iam::352980582205:user/mark@medthings.no",
    ]
    pull_servers = [
        "arn:aws:iam::352980582205:user/medthings-01",
    ]
}
app_docker_registry_names = ["ghcr.io", "352980582205.dkr.ecr.eu-central-1.amazonaws.com"]
app_docker_ecr_registry_address = "352980582205.dkr.ecr.eu-central-1.amazonaws.com"
app_docker_source_image_name = "ghcr.io/medthings/cerebellum-server:main"
