# Revolut DevOps Test

Welcome to the Revolut DevOps Test.

## Configuration

**Cli options**:

- `-h` - Print help message summary
- `--help` - Print more detailed help message
- `-a | --bind-address` - The address to bind the http server to (default: `[::1]:4200`)
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

> [!IMPORTANT]
> The current implementation of the storage backend doesn't support running multiple
> instances of the application at the same time. It is possible to tweak the code
> to support it, but it will require running the [TiKV](https://tikv.org/) or [FundationDB](https://www.foundationdb.org/)
> on separete instances.

By default, the application will store the data in the `.local/data` directory,
relative to the application working directory. To change the storage directory,
use the `--data-dir` cli option or the `REVOLUT_DATA_DIR` environment variable.
