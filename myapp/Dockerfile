#NATIVE COMPILATION DOCKERFILE
#
# ETAP 1: Budowanie aplikacji (Builder)
FROM rust:1.85-alpine AS builder

# Instalujemy musl-dev, ponieważ obrazy alpine wymagają go do kompilacji
RUN apk add --no-cache musl-dev

WORKDIR /app

# Sztuczka optymalizacyjna: Kompilujemy same zależności, aby Docker mógł je zapisać w cache.
# Dzięki temu zmiana kodu aplikacji nie powoduje ponownego pobierania bibliotek z sieci.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f src/main.rs target/release/deps/myapp*

# Kopiujemy właściwy kod źródłowy aplikacji
COPY src ./src

# Budujemy finalną wersję produkcyjną
RUN cargo build --release

# ==========================================
# ETAP 2: Uruchomienie (Finalny, lekki obraz)
# ==========================================
# GCR Distroless jest bezpieczny, lekki i zawiera minimalne biblioteki C (glibc/musl)
#FROM gcr.io/distroless/static-debian12:latest
#FROM alpine:latest
FROM scratch

WORKDIR /app

# Kopiujemy tylko gotowy plik binarny z etapu budowania
  COPY --from=builder /app/target/release/myapp .

# Definiujemy domyślny port dla kontenera
ENV PORT=8080
EXPOSE 8080

# Uruchamiamy aplikację
CMD ["./myapp"]
