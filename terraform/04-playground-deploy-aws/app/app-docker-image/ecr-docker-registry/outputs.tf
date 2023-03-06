output "docker_registry_url" {
    description = "Docker URL for this ECR registry"
    value = aws_ecr_repository.repo.repository_url
}