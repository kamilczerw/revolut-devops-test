terraform {
  required_providers {
    google = {
      source  = "hashicorp/google"
      version = "4.51.0"
    }
  }
}

module "gke" {
  source = "./modules/gke"

  cluster_name = "revolut-demo"
  region       = var.region

  # Create a GCR repository
  enable_gcr = true
}
