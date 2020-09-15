FROM rust:1.46 as builder
WORKDIR /usr/src/kpm
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src; echo 'fn main() {}' > src/main.rs && cargo build --release && rm -rf src

COPY src src
RUN cargo install --path .

FROM debian:buster-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/kpm /usr/local/bin/kpm
COPY templates templates
CMD ["kpm"]
