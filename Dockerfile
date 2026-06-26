FROM rust:1.93-slim

WORKDIR /app
COPY . .

ENTRYPOINT [ "cargo", "test", "--release", "--" ]