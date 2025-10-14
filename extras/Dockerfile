# Simple docker file to run Brush. Note that as Brush has very few dependencies, you should be able to run
# it on any system already.
#
# This image is for easier server deployments. Nb: This dockerfile explicitly installs nvidia drivers,
# I haven't tested this on non nvidia GPUs. Alternatively, you should be able
# to run with host drivers.
FROM rust:1.86 AS builder
RUN git clone https://github.com/ArthurBrussee/brush /brush
WORKDIR /brush
RUN cargo build --release -p brush-cli

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 1000 clouduser
USER clouduser
WORKDIR /home/clouduser

COPY --from=builder /brush/target/release/brush-cli /usr/local/bin/brush-cli

ENTRYPOINT ["brush-cli"]
