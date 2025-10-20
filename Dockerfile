FROM debian:bookworm-slim

WORKDIR /app
COPY . .

# Install base dependencies
ARG DEBIAN_FRONTEND=noninteractive
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        curl \
        make \
        pkg-config \
        gcc-arm-none-eabi \
        g++ \
        wget \
        git \
        nbc \
    && rm -rf /var/lib/apt/lists/*

# Set default mode (can override with --build-arg MODE=python)
ARG MODE=none

RUN if [ "$MODE" = "nxc" ]; then \
        make nxc; \
    elif [ "$MODE" = "python" ]; then \
        make python; \
    elif [ "$MODE" = "rust" ]; then \
        curl https://sh.rustup.rs -sSf | sh -s -- -y && \
        . $HOME/.cargo/env && \
        rustup install stable && \
        rustup default stable && \
        rustup target add armv5te-unknown-linux-musleabi && \
        make rust; \
    elif [ "$MODE" = "c++" ]; then \
        make c++; \
    elif [ "$MODE" = "test" ]; then \
        make test; \
        curl https://sh.rustup.rs -sSf | sh -s -- -y && \
        . $HOME/.cargo/env && \
        rustup install stable && \
        rustup update && \
        rustup default stable && \
        rustup target add armv5te-unknown-linux-musleabi; \
    else \
        echo "Invalid MODE. Use nxc, python, rust, or c++." && exit 1; \
    fi
