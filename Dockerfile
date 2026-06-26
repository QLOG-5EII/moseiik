FROM rust:latest

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY tests ./tests
COPY assets ./assets

ENTRYPOINT ["cargo", "test", "--release", "--"]