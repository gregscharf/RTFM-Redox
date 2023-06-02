# Use Ubuntu as the base image
FROM ubuntu:latest

# Install system dependencies
RUN apt-get update \
    && apt-get install -y \
        build-essential \
        libssl-dev \
        libxcb1-dev \
        libxcb-render0-dev \
        libxcb-shape0-dev \
        libxcb-xfixes0-dev

# Install Rustup
RUN apt-get install -y curl
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set the default Rust version
ENV PATH="/root/.cargo/bin:$PATH"
RUN rustup default stable

# Create a new directory for your Rust application
WORKDIR /app

# Copy your Rust application files to the container
COPY . .

# Build your Rust application
RUN cargo build --release

# Set the startup command for the container
CMD ["./target/release/redox]

