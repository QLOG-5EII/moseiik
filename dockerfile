# Utiliser l'image officielle de Rust en tant qu'image de base cela permet d'éviter l'intsalation de rust
FROM rust:latest

# Créer un répertoire de travail pour l'application
WORKDIR /app


# Cloner le projet depuis GitHub, il faut modifier l'url pour correspondre au bon repo
RUN git clone https://github.com/rcotte/moseiik.git .
# cela permet de télécharger et de dézipper les images dans /assets
RUN cd assets
RUN wget -q "https://filesender.renater.fr/download.php?token=178558c6-7155-4dca-9ecf-76cbebeb422e&files_ids=33679270" -O images.zip
RUN unzip -q images.zip #le -q permet d'eviter de print les étapes

# on build l'apps qu'une seul fois 
RUN cargo build --release

# Définir la commande par défaut pour exécuter l'application lorsque le conteneur démarre
# run les test 
ENTRYPOINT ["cargo", "test", "--release", "--"]

# run le projet
#ENTRYPOINT ["cargo", "run", "--release", "--"]

