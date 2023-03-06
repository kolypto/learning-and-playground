# This module will create an ECR Container Registry.
#
# There will be two groups of users:
# * Human users can push
# * Server users can pull
#
# Note that you must create a separate registry for every image!

# Docker Registry
resource "aws_ecr_repository" "repo" {
    # Name. Must be unique.
    name = var.registry_name

    # Delete the registry even if it contains images. Default: false
    force_delete = true

    # Name
    tags = { Name = "${var.registry_name} Images" }
}

# Registry Policy.
# Don't use JSON: here's a first class policy.
data "aws_iam_policy_document" "repo_policy" {
    statement {
        # Identifier
        sid = "Push for users"
        # "Allow" or "Deny"
        effect = "Allow"

        # See: `principals`, `resources` ; `not_principals`, `not_resources`

        # Principals: to whom the statement applies
        principals {
            # Type: "AWS", "Service", "Federated", "CanonicalUser", "*"
            type = "AWS"
            # List of identifiers.
            # With type = "AWS": IAM principal ARNs. See IAM Users: https://console.aws.amazon.com/iamv2/home#/users
            identifiers = var.registry_aws_iam_arns.push_users
        }

        # List of actions to Allow
        actions = [
            "ecr:ListImages",
            "ecr:PutImage",
            "ecr:BatchGetImage",
            "ecr:BatchDeleteImage",
            "ecr:DescribeImages",
            "ecr:GetDownloadUrlForLayer",  # Pre-signed URL
            "ecr:TagResource",
            "ecr:UntagResource",
            "ecr:InitiateLayerUpload",
            "ecr:UploadLayerPart",
            "ecr:CompleteLayerUpload",
            "ecr:DescribeRepositories",
            "ecr:ListTagsForResource",
            "ecr:BatchCheckLayerAvailability",
        ]
    }

    statement {
        sid = "Pull for servers"
        effect = "Allow"

        principals {
            type = "AWS"
            identifiers = var.registry_aws_iam_arns.pull_servers
        }

        actions = [
            "ecr:BatchGetImage",
            "ecr:ListImages",
            "ecr:DescribeImages",
            "ecr:ListTagsForResource",
            "ecr:BatchCheckLayerAvailability",
            "ecr:GetDownloadUrlForLayer",
            "ecr:DescribeRepositories",
            "ecr:DescribeImageScanFindings"

        ]
    }
}

# Associate the policy with the registry.
# Note that a registry can have only one policy!
resource "aws_ecr_repository_policy" "repo_policy" {
    repository = aws_ecr_repository.repo.name

    # Policy: JSON formatted string {"Statement": [...]}
    # It can be a literal JSON string, a `file()` interpolation, or "aws_iam_policy_document" data source.
    # NOTE: AWS IAM policy document supports its own "&{}"" interpolation syntax!
    policy = data.aws_iam_policy_document.repo_policy.json
}
