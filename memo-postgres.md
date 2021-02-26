### Benchmark Results

```
Inserts/second: ~13143.71336
Timelines/second: ~70.0280112
```

### System Information

I ran my benchmarks on a 13-inch, 2019, Macbook Pro, with all other applications closed. My Mac has the following stats:

```
Processor Name: Quad-Core Intel Core i7
Processor Speed: 2.8 GHz
Number of Processors:	1
Total Number of Cores:	4
Memory:	16 GB
```

### Database-related Stuff

For my database, I used a local installation of PostgreSQL 13.1, installed with the EDB installer from the following site: https://www.postgresql.org/download/macosx/, with the default storage engine.

The ORM that I used for the project, [Diesel](https://diesel.rs/), _I think_ uses prepared statements by default, though I am not sure – the only thing I could find was [this Reddit comment](https://www.reddit.com/r/rust/comments/7yuwor/caching_prepared_statements_in_sqlite_replicating/duje0zy?utm_source=share&utm_medium=web2x&context=3).

The SQL I used to create the tables can be found attached in the zip (under sql folder).

Additionally, I did not use secondary indexing.

### Data Generation

For my data generation, I decided to go with 10_000 users, and 200_000 followers. This would result in each person following 20 people on average, which I think is reasonable, and also means that each "person" tweeting will tweet about 100 tweets (I distributed the tweet user_ids over the same number of users).

I generated the follower records such that there are no duplicates, and no instances of someone following themselves (record where user_id = follows_id).

For generating tweets, I used a library called Faker to generate fake data. Each tweet had a tweet id, which was just a number increasing by 1 each time, a randomly-generated timestamp, a user id in the range 0..10_000, and a sentence with 1-10 words (Lorem Ipsum).

### Wrapup

Before I implemented this in Rust, I first wrote it in Typescript (Javascript). My Typescript stats were ~3.5k for inserts/sec, and ~20 for timelines/sec. I thought it was cool to see that my stats for TS were about 1/4 my stats for Rust, because being a single-threaded language, it makes sense that Rust was 4x faster, because
it was able to take advantage of all 4 cores!
