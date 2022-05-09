# Build and install
FROM rust:1.60 AS builder
WORKDIR /usr/src/rust-webservice-example
COPY . .
RUN cargo install --path .

# Copy the installed binary into a Debian image
# This is important so we don't end up with >1GB images that contain miscellaneous build files
FROM debian:bullseye-slim
RUN apt update && apt install -y libssl1.1 ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /opt/rust-webservice-example
COPY --from=builder /usr/local/cargo/bin/rust-webservice-example /opt/rust-webservice-example/rust-webservice-example

CMD ["./rust-webservice-example"]

EXPOSE 80