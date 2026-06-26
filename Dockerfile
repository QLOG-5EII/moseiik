# Image officielle Rust (multi-architecture)
FROM rust:1.88

# Répertoire de travail
WORKDIR /app

# Copier les fichiers Cargo
COPY Cargo.toml Cargo.lock ./

# Copier le code
COPY src ./src
COPY tests ./tests
COPY assets ./assets


# Compiler une première fois pour mettre les dépendances en cache
RUN cargo build --release

RUN wget https://nasext-vaader.insa-rennes.fr/ietr-vaader/moseiik_test_images.zip && \
    unzip moseiik_test_images.zip -d assets/tiles-small/images && \
    rm moseiik_test_images.zip

RUN cargo test --release

# Les paramètres supplémentaires seront passés après le --
ENTRYPOINT ["cargo", "test", "--release", "--"]
