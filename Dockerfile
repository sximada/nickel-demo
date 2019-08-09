FROM rust:latest
WORKDIR /app
COPY . .
RUN cargo install --path .
RUN cargo build --release
CMD ["/usr/local/cargo/bin/nickel-demo"]
