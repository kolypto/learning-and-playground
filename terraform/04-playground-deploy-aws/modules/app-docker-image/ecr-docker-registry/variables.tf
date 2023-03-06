variable "registry_name" {
  description = "Name of registry. Must be unique!"
  type = string
}

variable "registry_aws_iam_arns" {
  description = "Users who can: push images to the registry, and push images to the registry (AWS IAM ARNs)"
  type = object({
    # These users can push images (users)
    # Example: "arn:aws:iam::352980582205:user/human"
    push_users = list(string)

    # These users can pull images (servers)
    # Example: "arn:aws:iam::352980582205:user/server"
    pull_servers = list(string)
  })
}
