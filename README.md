# Revolut DevOps Test

Welcome to the Revolut DevOps Test.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) - The application is written
  in Rust programming language. To build the application, you need to have the Rust
  compiler installed on your machine.
- [Docker](https://docs.docker.com/get-docker/) - The application provides a Dockerfile
  to build the application in a Docker container. You can use Docker to build and
  run the application in a container.

## Local development

### Building

To build the project, run the following command:

```bash
cargo build
```

This will build the application in `debug` mode. The produced executable will be
stored at `./target/debug/revolut-devops-test`.

To build the application in `release` mode, run the following command:

```bash
cargo build --release
```

The produced executable can be found at `./target/release/revolut-devops-test`.

### Running

It is not required to build the project before running the application. You can
run the following command to build and run the applciation:

```bash
cargo run
```

After the application finishes compiling, it will start the http servers and can
accept the requests. You should see the output similar to the following:

```text
// TODO
```

### Testing

To run the tests, run the following command:

```bash
cargo test
```

## Docker

The application also provides docker support. This allows deploying the application
to Kubernetes or any other container orchestration platform.

To build the docker image, run the following command:

```bash
docker build -t revolut-devops-test .
```

Now you can run the docker container:

```bash
docker run -p 4200:4200 -p 4300:4300 revolut-devops-test
```

## Configuration

**Cli options**:

- `-h` - Print help message summary
- `--help` - Print more detailed help message
- `-a | --bind-address` - The address to bind the http server to (default: `[::1]:4200`)
- `--health-bind-address` - The address to bind the health server to (default: `[::1]:4300`)
- `-l | --log-level` - Log level for the application (default: `info`)
- `--log-encoder` - The format of the log output. It can be either `text` or `json`
  (default: `text`)
- `-d | --data-dir` - The directory to store the data (default: `.local/data`)

It is also possible to configure the application using the environment variables.
To do so, add the `REVOLUT_` prefix to the cli option name, use uppercase letters
and replace the `-` with `_`. For example, to set the log level, you can use the
`REVOLUT_LOG_LEVEL` environment variable.

## Storage

The application uses [SurrealDB](https://surrealdb.com/) as a storage backend. To
run the application, there are no dependencies required. The application will start
the storage backend automatically.

> [!NOTE]
> The current implementation of the storage backend doesn't support running multiple
> instances of the application at the same time. Which was one of the requirements
> of the assignment. The reason for this limitation is the choice of the storage
> backend used in the application. It is possible to tweak the code to support
> it, but it will require running the [TiKV](https://tikv.org/) or [FundationDB](https://www.foundationdb.org/)
> on separete instances.
>
> Running the storage backend on a separate instance will require additional
> configuration and will increase the complexity of the deployment.
>
> I would not recommend running such simple application with such specialized database.
> It would be better to use a managed database provided by the cloud provider.

By default, the application will store the data in the `.local/data` directory,
relative to the application working directory. To change the storage directory,
use the `--data-dir` cli option or the `REVOLUT_DATA_DIR` environment variable.

## Observability

The application provides the following observability features:

- **Logging** - Logs are written to the standard output and can be formatted as
  `text` or `json`. The default format is `text`. It is recommended to use the
  `json` format for structured logs when running the application in the cloud
  for better integration with the observability tools.
- **Metrics** - The application exposes the Prometheus metrics on the `/metrics`
  endpoint served on `4300` port by default.
- **Tracing** - The application supports a simple MDC-based tracing mechanism.
  The trace ID is generated for each request and can be found in the logs.
  Additionally, the trace ID can be passed in the `X-Request-ID` header to propagate
  the trace ID between the services.
- **Health check** - The application exposes the health check endpoint on the
  `/health` endpoint served on `4300` port by default.

## Repository structure

- `src/` - The source code of the application.
  - `setup/` - This directory contains all the initialization logic to start all
    the required services and tools. Such as the storage backend, logger and the
    http server.
  - `app/` - The main application logic. This directory contains the implementation
    of the http endpoint handlers and the business logic.
- `bin/` - The directory where all the executable files are stored.

## Final notes

There are a few things that I didn't have time to implement and should be part of
the final solution:

- **Github Actions** - The repo should contain the `.github/workflows` directory
  with the GitHub Actions workflows to build and test the application on each push
  to the repository.
  - The builds running on `main` should produce the docker image and push it to
    `ghcr.io` where it can be used by the deployment pipeline.
  - The builds running on the PRs should run the tests and the linter to ensure
    the code quality. If the tests fail, the PR should be blocked from merging.
- **Deployment** - 
