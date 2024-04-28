FROM rust:1.77 as builder
WORKDIR /usr/src/app

RUN rustup target add wasm32-unknown-unknown
RUN cargo install dioxus-cli@0.5.4

COPY . .
RUN dx build --release --features web
RUN cargo build --release --features server

FROM debian:bookworm-slim

WORKDIR /usr/local/bin
COPY --from=builder /usr/src/app/dist /usr/local/bin/dist
COPY --from=builder /usr/src/app/target/release/shopping-list-dioxus /usr/local/bin/

EXPOSE 8080
CMD ["shopping-list-dioxus"]
