FROM rust:1-slim-buster

RUN apt-get update -y && apt-get -y install ca-certificates pkg-config libssl-dev libfreetype6-dev cmake gcc autoconf && rm -rf /var/lib/apt/lists/*

COPY . /usr/src/api

WORKDIR /usr/src/api

EXPOSE 80:8000

RUN cargo build --release && cargo run