# Create the GKE cluster
module "gke" {
  source = "./modules/gke"

  cluster_name = "revolut-demo"
  region       = var.region

  # Create a GCR repository as part of the GKE module
  enable_gcr = true
}
