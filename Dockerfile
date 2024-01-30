FROM rust:1.75 as builder
WORKDIR /usr/src/shopping-list-dioxus
COPY . .

RUN rustup target add wasm32-unknown-unknown
RUN cargo install dioxus-cli
RUN dx build --features web --release

RUN cargo install --path . --features ssr

FROM debian:bookworm-slim
WORKDIR /usr/local/bin
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/shopping-list-dioxus /usr/local/bin/shopping-list-dioxus
COPY --from=builder /usr/src/shopping-list-dioxus/dist /usr/local/bin/dist
CMD ["shopping-list-dioxus"]
