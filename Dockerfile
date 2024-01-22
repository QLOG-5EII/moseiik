# Utilisez une image officielle Rust
FROM rust:latest

# Definir le répertoire de travail
WORKDIR /app

# Copier le code source dans le conteneur
COPY ./src /app/src
COPY ./assets /app/assets
COPY ./tests /app/tests

# Copier les fichier Cargo.toml
COPY Cargo.toml /app/Cargo.toml
COPY Cargo.lock /app/Cargo.lock


# Télécharger le contenu du lien Internet de images
ADD "https://filesender.renater.fr/download.php?token=178558c6-7155-4dca-9ecf-76cbebeb422e&files_ids=33679270" /app/assets/images.zip
RUN unzip /app/assets/images.zip -d /app/assets \
    && rm /app/assets/images.zip


# Télécharger les dépendances
RUN cargo build --release

# Point d'entrée pour exécuter cargo test avec des paramètres
ENTRYPOINT ["cargo", "test", "--release", "--"]

# Point d'entrée pour exécuter le fichier principal
CMD ["./target/release/moseiik"]





