data "google_project" "project" {}

# Create default service account for GKE
resource "google_service_account" "default" {
  account_id   = "gke-default-sa"
  display_name = "GKE Default Service Account"
}

# Define a Google Kubernetes Engine (GKE) cluster
resource "google_container_cluster" "gke" {
  name     = var.cluster_name
  location = var.region

  enable_autopilot = true

  # VPC_NATIVE enables ip aliasing which is required when useing autopilot
  networking_mode = "VPC_NATIVE"

  ip_allocation_policy {
    cluster_ipv4_cidr_block = "10.96.0.0/14"
  }

  node_config {
    service_account = google_service_account.default.email

    disk_size_gb = var.node_disk_size
  }

  # remove_default_node_pool = true
  # initial_node_count       = 1
}

# Define a Google Kubernetes Engine (GKE) node pool
# resource "google_container_node_pool" "primary_preemptible_nodes" {
#   name       = "${var.cluster_name}-node-pool"
#   location   = var.region
#   cluster    = google_container_cluster.primary.name
#   node_count = var.node_count
#
#   node_config {
#     preemptible  = true
#     machine_type = var.machine_type
#
#     disk_size_gb = var.node_disk_size
#
#     # Google recommends custom service accounts that have cloud-platform scope and permissions granted via IAM Roles.
#     service_account = google_service_account.default.email
#     oauth_scopes = [
#       "https://www.googleapis.com/auth/cloud-platform"
#     ]
#   }
# }
