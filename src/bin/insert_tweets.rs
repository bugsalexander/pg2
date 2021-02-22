use pg2::{
    deserialize_tweets, establish_postgres_pool, establish_redis_pool, insert_redis_tweet_1,
    insert_redis_tweet_2, insert_tweet, models::Tweet,
};
use std::{env, error::Error, time::Duration, time::Instant};
use tokio::runtime::{self, Runtime};

fn main() -> Result<(), Box<dyn Error>> {
    let runtime = runtime::Builder::new_multi_thread().build()?;
    let args: Vec<String> = env::args().collect();
    let mut tweets = deserialize_tweets()?;

    // sort the tweets in order of timestamp
    tweets.sort_by(|a, b| a.tweet_ts.cmp(&b.tweet_ts));

    let count: u64 = tweets.len() as u64;
    println!("inserting {} tweets!", count);

    // begin timer
    let start = Instant::now();

    if let Some(_mode) = args.get(1) {
        let mode: &str = _mode;
        match mode {
            "postgres" => run_postgres_insert(&runtime, tweets)?,
            "redis1" => run_redis_insert1(&runtime, tweets)?,
            "redis2" => run_redis_insert2(&runtime, tweets)?,
            _ => panic!("please provide either \"postgres\" or \"redis as mode argument\""),
        }
    } else {
        panic!("please provide either \"postgres\" or \"redis as mode argument\"");
    }

    runtime.shutdown_timeout(Duration::from_secs(120));

    let duration = start.elapsed();
    println!("inserting took {:#?}", duration);
    println!(
        "inserted at rate of {} tweets/second!",
        count / duration.as_secs()
    );

    Ok(())
}

fn run_postgres_insert(runtime: &Runtime, tweets: Vec<Tweet>) -> Result<(), Box<dyn Error>> {
    let pool = establish_postgres_pool();

    for tw in tweets {
        let pool2 = pool.clone();
        runtime.spawn_blocking(move || {
            let conn = pool2.get().unwrap();
            insert_tweet(&conn, tw);
        });
    }

    Ok(())
}

fn run_redis_insert1(runtime: &Runtime, tweets: Vec<Tweet>) -> Result<(), Box<dyn Error>> {
    let pool = establish_redis_pool();

    for tw in tweets {
        let pool2 = pool.clone();
        runtime.spawn_blocking(move || {
            let mut conn = pool2.get().unwrap();
            insert_redis_tweet_1(&mut conn, tw);
        });
    }

    Ok(())
}

fn run_redis_insert2(runtime: &Runtime, tweets: Vec<Tweet>) -> Result<(), Box<dyn Error>> {
    let pool = establish_redis_pool();

    for tw in tweets {
        let pool2 = pool.clone();
        runtime.spawn_blocking(move || {
            let mut conn = pool2.get().unwrap();
            insert_redis_tweet_2(&mut conn, tw);
        });
    }

    Ok(())
}
