# Use the official Rust image as the base
FROM rust:1.84.1

# Install necessary libraries
RUN apt-get update && apt-get install -y mc

# Install necessary dependencies for cross-compilation
RUN apt-get update && \
    apt-get install -y mingw-w64 && \
    rustup target add x86_64-pc-windows-gnu && \
    apt-get clean && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory inside the container
WORKDIR /app

CMD ["/bin/bash"]