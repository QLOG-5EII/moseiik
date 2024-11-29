# Step 1: Build
# Use the official Rust slim-bullseye image as the base for the build stage.
FROM rust:slim-bullseye AS builder
LABEL authors="Marie Dubouays - Thomas Lesaulnier"

# Update the package index, install necessary tools, and clean up afterwards to reduce image size.
RUN apt-get update && apt-get install -y curl unzip build-essential && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container for building the project.
WORKDIR /app/moseiik

# Copy the manifest and lock files for Rust dependency management.
COPY Cargo.toml Cargo.lock ./

# Copy the source code and assets into the container.
COPY src ./src
COPY assets ./assets

# Fetch dependencies and build the project in release mode.
RUN cargo fetch && cargo build --release

# Step 2: Runtime
# Use a minimal Debian slim image for the runtime stage.
FROM debian:bullseye-slim AS runtime

# Update the package index, install required runtime dependencies, and clean up afterwards.
RUN apt-get update && apt-get install -y libssl1.1 && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

# Copy the built binary and assets from the builder stage.
COPY --from=builder /app/moseiik/target/release/moseiik /moseiik
COPY --from=builder /app/moseiik/assets /assets

# Set the working directory to the root.
WORKDIR /

# Define the default command for the container.
ENTRYPOINT ["/moseiik"]

# Provide a default argument to display the help message.
CMD ["--help"]
