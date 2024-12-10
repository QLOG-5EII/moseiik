# Set up the OS for docker image
FROM ubuntu:latest

# Install dependencies
RUN apt-get update 
RUN apt-get install -y --no-install-recommends 	\
    git 					\
    wget 					\
    unzip 					\
    curl 					\
    build-essential 				\
    ca-certificates
RUN rm -rf /var/lib/apt/lists/*
    
# Install Rust and Cargo via rustup
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add Cargo to PATH
ENV PATH="/root/.cargo/bin:${PATH}"

# Switch working directory
WORKDIR /app

# Donwload images for test into /assets
RUN wget https://nasext-vaader.insa-rennes.fr/ietr-vaader/moseiik_test_images.zip -P tests/
RUN unzip tests/moseiik_test_images.zip -d assets/
RUN rm tests/moseiik_test_images.zip

# Install git repo
#RUN git clone https://github.com/PapyPouley/moseiik.git .

# Move to docker branch (only for dockerfile testing)
#RUN git checkout UnitTest

# Build and run unit tests
#ENTRYPOINT [ "cargo", "test", "--release", "--" ]

# Create a script and fill it with commands in order to do the following actions during the docker run :
#	- Install git repo
#	- Move to docker branch (only for dockerfile testing)
#	- Build and run tests


# Run a script that 
ENTRYPOINT [ "bash", "-c", "git clone https://github.com/PapyPouley/moseiik.git . && cd moseiik && git checkout UnitTest && cargo test --release --" ]
	  
	  
	  
	  
	  
