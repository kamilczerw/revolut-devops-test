variable "region" {
  type        = string
  description = "AWS region where the EKS cluster will be created"
}

variable "cluster_name" {
  type        = string
  description = "Name of the EKS cluster to be created"
}

variable "machine_type" {
  type        = string
  description = "The type of the machine to use for GKE cluster nodes"

  default = "e2-medium"
}

variable "node_count" {
  type        = number
  description = "The number of nodes to create in the GKE cluster"

  default = 1
}

variable "node_disk_size" {
  type        = number
  description = "The disk size (in GB) of the nodes in the GKE cluster"

  default = 10
}

variable "enable_gcr" {
  type        = bool
  description = "Create a GCR repository for the GKE cluster"

  default = false
}
