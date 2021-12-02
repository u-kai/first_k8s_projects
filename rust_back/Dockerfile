FROM rust:1.55.0 as builder
RUN USER=root cargo new --bin rust_back
WORKDIR /rust_back
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
RUN rm ./target/release/deps/rust_back*
RUN cargo build --release

FROM rust:1.55.0
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /rust_back/target/release/rust_back .
CMD ["./rust_back"]

EXPOSE 8081
