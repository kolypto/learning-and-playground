output infrastructure {
    value = {
        server_ssh_user     = data.terraform_remote_state.infrastructure.outputs.server_ssh_user
        server_public_ip    = data.terraform_remote_state.infrastructure.outputs.server_public_ip
        project_name        = data.terraform_remote_state.infrastructure.outputs.project_name
        postgres_psql_root  = data.terraform_remote_state.infrastructure.outputs.postgres_psql_root
    }
}
