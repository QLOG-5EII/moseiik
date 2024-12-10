# Base image
FROM ubuntu:latest

# Install dependencies
RUN apt-get update \
    && apt-get install -y --no-install-recommends \
       git \
       wget \
       unzip \
       curl \
       build-essential \
       ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install Rust and Cargo via rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Cargo to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Set the working directory
WORKDIR /app

# Download and extract test images into /assets
RUN wget https://nasext-vaader.insa-rennes.fr/ietr-vaader/moseiik_test_images.zip -P /tmp \
    && unzip /tmp/moseiik_test_images.zip -d /assets \
    && rm /tmp/moseiik_test_images.zip

# Set entrypoint to clone the repo, move assets, and run tests
ENTRYPOINT ["bash", "-c", "\
git clone https://github.com/PapyPouley/moseiik.git /app/moseiik && \
mv /assets /app/moseiik/assets && \
cd /app/moseiik && \
cargo test --release --"]
