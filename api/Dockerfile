FROM rust:1.69 AS chef
RUN cargo install cargo-chef --locked
# sqlxのoffline-modeを使用するためにcliを入れる
RUN cargo install sqlx-cli --no-default-features --features native-tls,mysql
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .

FROM builder AS tmp_builder
ARG SQLX_OFFLINE=true
ENV SQLX_OFFLINE=$SQLX_OFFLINE
RUN cargo build --release --bin api

# We do not need the Rust toolchain to run the binary!
FROM debian:buster-slim AS runtime
ARG DATABASE_URL
ENV DATABASE_URL=$DATABASE_URL
WORKDIR /app
COPY --from=tmp_builder /app/target/release/api /usr/local/bin/app
# error while loading shared libraries: libssl.so.1.1: cannot open shared object file: No such file or directory が出るので、opensslを入れる
RUN apt-get update && apt install -y openssl
CMD ["/usr/local/bin/app"]
EXPOSE 8080