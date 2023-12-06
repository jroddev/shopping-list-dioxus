```
# build client
dx build --features web --release

#build and run server
cargo run --features ssr --release
```

## Postgres

Based off https://github.com/jroddev/sqlx-example

```
cargo install sqlx-cli

# create a nenw DB migrations
sqlx-cli migrate add <migration name>
```
