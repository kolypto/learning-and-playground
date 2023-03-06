
# Remote state
variable "remote_state_s3_bucket" {
    description = "The bucket to read the remote state from"
    type = string
}
variable "remote_state_infrastructure" {
    description = "Path to the state file in the bucket"
    type = string
}
