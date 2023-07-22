FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/app/target \
    cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y libpq-dev
COPY --from=builder /usr/local/cargo/bin/rust-admin-api /usr/local/bin/rust-admin-api
CMD ["rust-admin-api"]