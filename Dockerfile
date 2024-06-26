# syntax=docker/dockerfile:1.4

#######################################
##  Stage 1: Build the Rust project  ##
#######################################
FROM rust:1.79 as build


# Set the working directory
WORKDIR /build

# Install libclang
# This is required by the surrealdb crate.
RUN apt-get update && apt-get install -y llvm-dev libclang-dev clang

# Copy only Cargo.toml and Cargo.lock to cache dependencies
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Create a dummy lib.rs file to build the dependencies. This is done to speed up
# the build process by caching the dependencies before copying the source code.
RUN mkdir src/ && \
  echo "fn dummy() {println!(\"Dummy lib.rs\")}" > src/lib.rs && \
  cargo build --release && \
  rm src/lib.rs

# Now that the dependencies are built, copy your source code
COPY ./src ./src

# Build the project with the actual source code
RUN cargo build --release


#######################################
##  Stage 2: Create the final image  ##
#######################################
FROM ubuntu:24.04

# Set the working directory
WORKDIR /app

RUN useradd -m -r revolut && \
  mkdir /app/data && \
  chown -R revolut:revolut /app

# Create a new user to run as non-root
USER revolut:revolut

# Set the environment variables
ENV REVOLUT_DATA_DIR=/app/data
ENV REVOLUT_LOG_ENCODER=json

# Copy the compiled binary from the builder stage
COPY --from=build /build/target/release/revolut-devops-test /usr/local/bin/revolut-devops-test

# Expose the port that the server will run on
EXPOSE 4200

# Expose the healthcheck port
EXPOSE 4300

# Create a volume for the data
VOLUME /app/data

ENTRYPOINT ["/usr/local/bin/revolut-devops-test"]
