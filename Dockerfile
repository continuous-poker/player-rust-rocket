FROM rust:1.73.0 AS builder
WORKDIR /scripts
ADD src src
ADD Cargo.toml .
ADD Cargo.lock .
ADD Rocket.toml .
ENV RUSTFLAGS="-A dead_code"
RUN cargo build --release
ENTRYPOINT ["target/release/player-rust-rocket"]
