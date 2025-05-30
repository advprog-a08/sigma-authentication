FROM rust:1.86-alpine AS builder

RUN apk add --no-cache musl-dev gcc make pkgconfig openssl-dev openssl-libs-static protoc protobuf-dev

WORKDIR /app

COPY . .

ENV SQLX_OFFLINE=true

RUN rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:latest

RUN apk add --no-cache libgcc

WORKDIR /app

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/sigma-authentication .

EXPOSE 50051
EXPOSE 8082

CMD [ "./sigma-authentication" ]
