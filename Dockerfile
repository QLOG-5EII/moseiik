FROM rust:1.76-bullseye

RUN apt-get update && apt-get install -y wget unzip && rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN mkdir -p assets/images

RUN wget -q https://nasext-vaader.insa-rennes.fr/ietr-vaader/moseiik_test_images.zip -O /tmp/images.zip && \
    unzip -q /tmp/images.zip -d assets/ && \
    rm /tmp/images.zip

COPY Cargo.toml Cargo.lock* ./

COPY . .

ENTRYPOINT [ "cargo", "test", "--release", "--" ]