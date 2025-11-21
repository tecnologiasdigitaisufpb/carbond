FROM ghcr.io/rust-cross/rust-musl-cross:x86_64-musl

# Copy src into container
COPY . /home/rust/src
WORKDIR /home/rust/src

# Build src in pre-configured musl build environment
RUN cargo build --release

# Create required folder structure for config
RUN mkdir -p /etc/carbond

# CD to the folder with the created binaries
WORKDIR /home/rust/src/target/x86_64-unknown-linux-musl/release