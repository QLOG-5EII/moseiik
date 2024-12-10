# Étape unique pour la construction
FROM rust:slim-bullseye AS builder
LABEL authors="Marie Dubouays - Thomas Lesaulnier"

# Variables d'environnement
ENV CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH

# Installation des outils nécessaires
RUN apt-get update && apt-get upgrade -y \
    curl unzip build-essential libssl-dev && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Répertoire de travail
WORKDIR /app/moseiik

# Copie des fichiers
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY assets ./assets
COPY tests ./tests

# Téléchargement et extraction d'images de test
RUN curl -o vignettes.zip https://nasext-vaader.insa-rennes.fr/ietr-vaader/moseiik_test_images.zip && \
    unzip vignettes.zip -d ./vignettes && \
    rm vignettes.zip

# Pré-téléchargement des dépendances et compilation en mode release
RUN cargo fetch && cargo build --release

# Lancement des tests par défaut
ENTRYPOINT [ "cargo", "test", "--release", "--" ]
