FROM rust:1.90-bookworm AS build

WORKDIR /app

COPY ./certs ./certs
COPY ./Cargo.lock .
COPY ./Cargo.toml .
COPY ./src ./src
COPY ./.env .env
COPY ./.sqlx .sqlx
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM debian:bookworm-slim AS deploy

WORKDIR /app
COPY ./certs ./certs
COPY ./templates ./templates
COPY ./.env .env

COPY --from=build app/target/release/toy-app .
ENTRYPOINT ["./toy-app"]
