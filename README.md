# Realworld

## Setup Database

```sh
psql -f setup.sql
cargo install diesel_cli --no-default-features --features "postgres"
diesel migration run
```

## Run

```sh
cargo run
```

Application load configuration from `.env`.


## Test

test with [Realworld API Test sh](https://github.com/gothinkster/realworld/tree/master/api)

