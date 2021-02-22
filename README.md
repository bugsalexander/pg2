# Data Storage Benchmark

A Twitter-based benchmark to compare the performances of a traditional relational databases and key value stores.

## Setup and Installation

This project expects local installations of Redis and/or PostgreSQL. Please put a `.env` file in the root directory with the following format:

```
DATABASE_URL=postgres://username:password@localhost/dbname
REDIS_URL=redis://localhost
```

For running auto-migrations, you will also need to install the diesel cli, which can be done with `cargo install diesel_cli`. For just Postgres support, run it with the `--no-default-features --features postgres` flags. After doing so, you may run the migrations to create the necessary tables with `diesel migration run`.

## Running the benchmarks

First, you will need to generate fake JSON tweets and followers, as well as insert the followers. To do so, run the following commands:

- `cargo run --bin generate_tweets`
- `cargo run --bin generate_followers`
- `cargo run --bin insert_followers (postgres|redis1|redis2)`

Next, you can run the various benchmarks. To make sure your benchmark results are consistent, I suggest closing all other applications besides your terminal, however this is up to you. The benchmarks consist of measuring time to insert 1_000_000 tweets, as well as the time to query home timelines, however the amount of home timelines queried depends on the implementation (as the speeds are relatively different).

- `cargo run --bin insert_tweets (postgres|redis1|redis2)`
- `cargo run --bin query_timelines (postgres|redis1|redis2)`

Also note that to run the separate redis benchmarks (strategies 1 and 2 specifically), you must flush Redis before re-inserting data.

## Data Modeling

For details on data modeling for Postgres see the [memo-postgres.md](./memo-postgres.md) file. For details on data modeling for Redis, see [memo-redis.md](./memo-redis.md).
