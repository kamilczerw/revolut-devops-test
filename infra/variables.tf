variable "project_id" {
  type        = string
  description = "The GCP project ID. The project ID can be fetched by running `gcloud projects list`"
}

variable "region" {
  type        = string
  description = "The GCP region"

  default = "europe-west1"
}
