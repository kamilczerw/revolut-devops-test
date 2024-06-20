output "gke_cluster_name" {
  value = module.gke.cluster_name
}

output "gcr_repository_url" {
  value       = module.gke.gcr_repository_url
  description = "The URL of the GCR repository"
}
