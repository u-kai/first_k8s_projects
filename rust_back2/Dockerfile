FROM rust:1.55.0 as builder
RUN USER=root cargo new --bin rust_back2
WORKDIR /rust_back2
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
RUN rm ./target/release/deps/rust_back2*
RUN cargo build --release

FROM rust:1.55.0
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /rust_back2/target/release/rust_back2 .
CMD ["./rust_back2"]

EXPOSE 8081
