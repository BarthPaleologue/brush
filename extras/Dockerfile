# Simple docker file to run Brush headlessly. This image builds the CLI runner so the GPU
# pipeline can execute without any windowing dependencies.
FROM rust:1.86 AS builder
WORKDIR /workspace
COPY Cargo.toml Cargo.lock cubecl.toml dist-workspace.toml brush_blueprint.rbl ./
COPY .cargo ./.cargo
COPY crates ./crates
COPY examples ./examples
COPY extras ./extras
RUN cargo build --release -p brush-runner

FROM ubuntu:22.04

# Install dependencies.
RUN apt-get update && DEBIAN_FRONTEND="noninteractive" apt-get install -y \
    build-essential \
    cmake \
    libvulkan1 \
    vulkan-tools \
    mesa-vulkan-drivers \
    nvidia-driver-570 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir /workdir
COPY --from=builder /workspace/target/release/brush-runner /usr/local/bin/brush-runner
WORKDIR /workdir

# Below command creates home dir for 1000 UID user
RUN useradd -m -u 1000 clouduser
RUN chown -R 1000:root /workdir && chmod -R 775 /workdir

# Launch training.
USER clouduser
ENTRYPOINT ["brush-runner"]
