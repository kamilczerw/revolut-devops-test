# Create GCR repository
resource "google_artifact_registry_repository" "gcr" {
  count = var.enable_gcr ? 1 : 0

  location      = var.region
  repository_id = var.cluster_name
  description   = "Docker registry for the ${var.cluster_name} GKE cluster"
  format        = "DOCKER"
}
#
# resource "google_service_account" "default" {
#   count = var.enable_gcr ? 1 : 0
#
#   account_id   = "${var.cluster_name}-gcr-sa"
#   display_name = "Service Account for GCR access"
# }

resource "google_project_iam_member" "gcr_storage_admin" {
  count = var.enable_gcr ? 1 : 0

  project = data.google_project.project.project_id
  role    = "roles/storage.admin"
  member  = "serviceAccount:${google_service_account.default.email}"
}

resource "google_project_iam_member" "gcr_artifact_registry_reader" {
  count = var.enable_gcr ? 1 : 0

  project = data.google_project.project.project_id
  role    = "roles/artifactregistry.reader"
  member  = "serviceAccount:${google_service_account.default.email}"
}
