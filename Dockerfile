FROM rust:1.46 as builder
WORKDIR /usr/src/kpm

# First build dependecies only, so those artifacts can be cached in image.
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src; \
    echo 'fn main() {}' > src/main.rs && \
    cargo build --release && \
    rm -rf src

# Then build the app
COPY src src
COPY style style
COPY buildinfo.conf buildinfo.conf
# The touch command in this script is to make sure the app itself is
# actually rebuilt after the dependency build with an empty main.
RUN \
    sed -e 's/^/export /' -e 's/=/="/' -e 's/$/"/' buildinfo.conf > buildinfo.sh && \
    . ./buildinfo.sh && \
    touch src/main.rs && \
    cargo install --path .

# Then start a new slim image (without dev tools) and copy in the binary
FROM debian:buster-slim

# Install libssl and required standard certificates
RUN apt-get update && \
    apt-get install -y libssl1.1 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/kpm /usr/local/bin/kpm

# The Tera templates are needed at runtime.
COPY templates templates
CMD ["kpm"]
