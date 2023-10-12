# Use an official Rust image as a parent image
FROM rust:1.55 as builder

# Set the working directory
WORKDIR /usr/src/framed_pay

# Copy the local project to the container
COPY . .

# Build the project
RUN cargo build --release

# Use an official Debian slim image for the runtime environment
FROM debian:buster-slim

# Copy the binary to the container
COPY --from=builder /usr/src/framed_pay/target/release/framed_pay /usr/local/bin/framed_pay

# Run the application when the container launches
CMD ["framed_pay"]
