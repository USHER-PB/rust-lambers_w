FROM rust:1.85 as builder  

# Create a new directory for the app
WORKDIR /usr/src/lambers_w

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create a new directory for the source code
COPY src ./src

# Build the application
RUN cargo build --release

# Use a base image with the required glibc version
FROM ubuntu:22.04 

# Install required libraries
RUN apt-get clean ; apt-get update ; apt-get install -y libc6 ; rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/lambers_w/target/release/lambers_w /usr/local/bin/lambers_w

# Set the entrypoint
ENTRYPOINT ["/usr/local/bin/lambers_w"]
