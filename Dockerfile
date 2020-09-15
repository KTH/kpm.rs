FROM rust:1.46 as builder
WORKDIR /usr/src/kpm
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/kpm /usr/local/bin/kpm
CMD ["kpm"]