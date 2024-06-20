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

  # We can use the autopilot to manage the cluster so we don't have to worry about it too much.
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
}
