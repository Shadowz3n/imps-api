FROM rust:1.40 as builder
WORKDIR /usr/src/imps-api
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update -y && apt-get install -y extra-runtime-dependencies
COPY --from=builder /usr/local/cargo/bin/imps-api /usr/local/bin/imps-api
CMD ["imps-api"]