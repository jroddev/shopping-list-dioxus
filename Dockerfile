FROM rust:1.75 as builder
WORKDIR /usr/src/shopping-list-dioxus
COPY . .
RUN cargo install --path . --features ssr
RUN ls /usr/local/cargo/bin/

FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
RUN ls /usr/local/bin/
COPY --from=builder /usr/local/cargo/bin/shopping-list-dioxus /usr/local/bin/shopping-list-dioxus
CMD ["shopping-list-dioxus"]
