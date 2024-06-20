output "cluster_name" {
  value = google_container_cluster.gke.name
}

output "cluster_endpoint" {
  value = google_container_cluster.gke.endpoint
}

output "gcr_repository_url" {
  value       = "${var.region}-docker.pkg.dev/${data.google_project.project.project_id}/${google_artifact_registry_repository.gcr[0].repository_id}"
  description = "The URL of the GCR repository"
}
