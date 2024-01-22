FROM ubuntu:latest

# Installation of the libraries
RUN apt-get update && \
    apt-get install -y &&\
    apt install -y unzip &&\
    apt install -y wget  &&\
    apt install cargo -y
    

# Working directory
WORKDIR /app

# Adding all the files to the container
COPY ./src /app/moseiik

#telechargement des images de test
RUN wget "https://filesender.renater.fr/download.php?token=178558c6-7155-4dca-9ecf-76cbebeb422e&files_ids=33679270" -O images.zip && \

RUN unzip images.zip && mv images ./assets

WORKDIR /app

# Build the project
RUN cargo build --release

# The parameters to the command
ENTRYPOINT ["cargo", "test", "--release"]
