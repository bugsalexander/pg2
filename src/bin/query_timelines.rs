use pg2::{establish_connection_pool, insert_tweet, read_tweets_from_file};
use rand::{Rng, rngs::SmallRng, thread_rng};
use std::time::Duration;
use std::{error::Error, time::Instant};
use tokio::runtime;

fn main() -> Result<(), Box<dyn Error>> {
    let runtime = runtime::Builder::new_multi_thread().build()?;

    println!("querying timelines!");

    let pool = establish_connection_pool();

    // begin timer and rng
    let end = Duration::from_secs(60);
    let rng = thread_rng();

    unimplemented!();
    
    // while (Instant::now() < end) {
    //     let pool2 = pool.clone();
    //     let uid = rng.gen_range(0..20_000);
    //     runtime.spawn_blocking(move || {
    //         let conn = pool2.get().unwrap();
    //         query_timeline(&conn);
    //     });
    // }

    // runtime.shutdown_timeout(Duration::from_secs(120));

    // println!("inserting took {:#?}", );

    // Ok(())
}
