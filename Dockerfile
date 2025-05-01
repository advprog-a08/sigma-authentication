FROM rust:1.86-alpine AS builder

RUN apk add --no-cache musl-dev gcc make openssl-dev

WORKDIR /app

COPY . .

RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest

RUN apk add --no-cache libgcc

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/sigma-authentication .

ENV ROCKET_ENV=production
ENV ROCKET_ADDRESS=0.0.0.0

EXPOSE 8000

CMD [ "./sigma-authentication" ]
