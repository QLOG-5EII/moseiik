# Base image
FROM alpine:latest

# Installer les dépendances nécessaires
RUN apk add --no-cache \
    bash \
    wget \
    unzip \
    curl \
    build-base \
    ca-certificates \
    && update-ca-certificates

# Installer Rust via rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Ajouter Cargo au PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Définir le répertoire de travail
WORKDIR /app/moseiik

# Copier les fichiers sources
COPY ../Cargo.toml ../Cargo.lock ./
COPY ../src ./src
COPY ../assets ./assets
COPY ../tests ./tests

# Télécharger et extraire les images de test
RUN mkdir ./tmp \
    && wget https://nasext-vaader.insa-rennes.fr/ietr-vaader/moseiik_test_images.zip -P ./tmp \
    && unzip ./tmp/moseiik_test_images.zip -d /app/assets \
    && rm ./tmp/moseiik_test_images.zip

# Copier les images dans le bon dossier et nettoyer
RUN cp -r /app/assets/images ./assets \
    && rm -rf /app/assets

# Définir l'entrée du conteneur
ENTRYPOINT ["cargo", "test", "--release", "--"]
