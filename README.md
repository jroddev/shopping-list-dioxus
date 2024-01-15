```
# build client
cargo install dioxus-cli
dx build --features web --release

#build and run server
cargo run --features ssr --release
```

## Postgres

Based off https://github.com/jroddev/sqlx-example

```
# start Postgres
podman-compose up -d

# install sqlx cli tool
cargo install sqlx-cli

# create a nenw DB migrations
sqlx-cli migrate add <migration name>
```
