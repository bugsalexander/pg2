use pg2::{establish_connection_pool, insert_tweet, read_tweets_from_file};
use std::{time::Duration};
use std::{error::Error, time::Instant};
use tokio::{runtime};

fn main() -> Result<(), Box<dyn Error>> {
    let runtime = runtime::Builder::new_multi_thread().build()?;
    
    let tweets = read_tweets_from_file()?;

    println!("inserting {} tweets!", tweets.len());

    let pool = establish_connection_pool();

    // begin timer
    let start = Instant::now();

    for tw in tweets {
        let pool2 = pool.clone();
        runtime.spawn_blocking(move || {
            let conn = pool2.get().unwrap();
            insert_tweet(&conn, tw);
        });
    }

    runtime.shutdown_timeout(Duration::from_secs(120));
    
    println!("inserting took {:#?}", start.elapsed());

    Ok(())
}
