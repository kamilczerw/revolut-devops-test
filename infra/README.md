# Infrastructure

Here you can find all the necessary information related to the provisioning of the
infrastructure to run the application on Google Cloud Platform using Kubernetes.

## Requirements

- [gcloud CLI](https://cloud.google.com/sdk/docs/install): To authenticate terraform
  to interact with the Google Cloud Platform.
- [terraform CLI](https://developer.hashicorp.com/terraform/install?product_intent=terraform):
  To provision the infrastructure on Google Cloud Platform.
- [kubectl CLI](https://kubernetes.io/docs/tasks/tools/): To interact with the Kubernetes
  cluster.

## Initial setup

Once you have the `gcloud` and `terraform` installed, you need to create a new project
on Google Cloud Platform. Copy the project ID and put it in the `local.auto.tfvars`
in the `infra/` folder.

You will also need to have the `gke-cloud-auth-plugin` to interact with the Kubernetes
cluster. To install it run the following command:

```bash
gcloud components install gke-gcloud-auth-plugin
```

> [!TIP]
> Using `.auto.tfvars` extension will automatically load the variables when running
> the terraform commands.

```bash
export PROJECT_ID="your-project-id"
echo "project_id = \"$PROJECT_ID\"" > ./local.auto.tfvars
```

Now, authenticate the `gcloud` CLI with your Google Cloud Platform account

```bash
gcloud auth login
```

And set the project ID to the one you created earlier

```bash
gcloud config set project "$PROJECT_ID"
gcloud config set compute/region "europe-west1"
```

> [!IMPORTANT]
> Make sure that all the APIs are enabled for the project. You can enable them using
> the following command:
>
> ```bash
>  gcloud services enable \
>    container.googleapis.com \
>    containerregistry.googleapis.com
> ```

Now, that you have everything setup, you can provision the infrastructure using the
terraform apply command. Run the following command:

```bash
terraform apply
export CLUSTER_NAME=$(terraform output -raw gke_cluster_name)
```

Once the cluster you need to configure the `kubectl` to interact with the cluster.

```bash
gcloud container clusters get-credentials "$CLUSTER_NAME"
```

This should be enough to interact with the Kubernetes cluster.

## Docker registry

The GKE module provides a way to setup the Google Container Registry to store the
docker iamges. It is enabled by default. You can disable it by setting the `enable_gcr`
to `false` in the `gke` module configuration.

To be able to push the docker images to the Google Container Registry, you need to
authenticate the docker CLI with the Google Cloud Platform.

```bash
gcloud auth configure-docker; gcloud auth configure-docker europe-west1-docker.pkg.dev
export REGISTRY="$(terraform output -raw gcr_repository_url)"
```

Now you can build the docker image and push it to the Google Container Registry.

```bash
docker build -t "$REGISTRY/revolut-demo:v0.1.0" .
dcoker push "$REGISTRY/revolut-demo:v0.1.0"
```
