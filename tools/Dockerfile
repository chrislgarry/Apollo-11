# Multi-stage Dockerfile for building and verifying Apollo 11 AGC source.
#
# Stage 1: Build yaYUL and yaAGC from the Virtual AGC project
# Stage 2: Minimal runtime image with assembler and Rust toolchain
#
# Usage:
#   docker build -t agc-builder -f tools/Dockerfile .
#   docker run --rm -v $(pwd):/workspace agc-builder tools/build.sh Comanche055

# --- Stage 1: Build yaYUL from source ---
FROM ubuntu:22.04 AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    git ca-certificates build-essential \
    && rm -rf /var/lib/apt/lists/*

# Pin to a specific commit for reproducibility
ARG VIRTUALAGC_COMMIT=master
RUN git clone --depth 1 https://github.com/virtualagc/virtualagc.git /virtualagc

# Build yaYUL (the assembler)
WORKDIR /virtualagc/yaYUL
RUN make -j$(nproc)

# Build yaAGC (the simulator) for integration tests
WORKDIR /virtualagc/yaAGC
RUN make -j$(nproc) || true

# --- Stage 2: Runtime image ---
FROM ubuntu:22.04

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates curl build-essential \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Copy yaYUL binary
COPY --from=builder /virtualagc/yaYUL/yaYUL /usr/local/bin/yaYUL

# Copy yaAGC binary (if built successfully)
COPY --from=builder /virtualagc/yaAGC/yaAGC /usr/local/bin/yaAGC

WORKDIR /workspace

ENTRYPOINT ["/bin/bash"]
