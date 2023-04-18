# build stage
FROM --platform=$BUILDPLATFORM lukemathwalker/cargo-chef:latest-rust-bullseye AS chef
WORKDIR /build/

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
    libprotobuf-dev \
    protobuf-compiler \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

# install zig
RUN curl -L "https://ziglang.org/download/0.10.1/zig-linux-$(uname -m)-0.10.1.tar.xz" | tar -J -x -C /usr/local && \
    ln -s "/usr/local/zig-linux-$(uname -m)-0.10.1/zig" /usr/local/bin/zig

# install zigbuild
RUN cargo install --locked cargo-zigbuild

# install targets
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /build/recipe.json recipe.json
ARG TARGETPLATFORM

# FIXME: Merge this case statement and the one below into a build script
ARG TARGETPLATFORM
RUN case "$TARGETPLATFORM" in \
    "linux/arm64") target="aarch64-unknown-linux-gnu" ;; \
    "linux/amd64") target="x86_64-unknown-linux-gnu" ;; \
    esac \
    && rustup target add "$target" \
    && cargo chef cook --zigbuild \
    --release \
    --target "$target" \
    --target-dir target/release \
    --recipe-path recipe.json
COPY . .
RUN case "$TARGETPLATFORM" in \
    "linux/arm64") target="aarch64-unknown-linux-gnu" ;; \
    "linux/amd64") target="x86_64-unknown-linux-gnu" ;; \
    esac \
    && cargo zigbuild --release \
    --target "$target" \
    --target-dir target/release \
    --bin relayer

FROM gcr.io/distroless/cc
WORKDIR /app/
COPY --from=builder /build/target/release/relayer /usr/local/bin/relayer
ENTRYPOINT ["/usr/local/bin/relayer"]
