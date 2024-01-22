# On utilise l'image officielle Rust pour avoir tout ce qu'il faut pour exécuter les tests.
FROM rust:latest
WORKDIR /app

# On clone le dépôt Git
RUN git clone https://github.com/JessyQLOG/moseiik.git

# On lance les tests en précisant où se trouve notre fichier Cargo.toml
RUN cargo test --manifest-path=/app/moseiik/Cargo.toml --release --
