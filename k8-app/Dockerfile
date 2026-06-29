FROM rust:1.85-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f src/main.rs target/release/deps/myapp*

COPY src ./src

RUN cargo build --release

#FROM gcr.io/distroless/static-debian12:latest
#FROM alpine:latest
FROM scratch

WORKDIR /app

COPY --from=builder /app/target/release/k8-app .

ENV PORT=8080
EXPOSE 8080

CMD ["./k8-app"]
