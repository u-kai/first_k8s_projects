FROM rust:1.55.0 as builder
RUN USER=root cargo new --bin table_job_rust
WORKDIR /table_job_rust
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
RUN rm ./target/release/deps/table_job_rust*
RUN cargo build --release

FROM rust:1.55.0
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /table_job_rust/target/release/table_job_rust .
CMD ["./table_job_rust"]
