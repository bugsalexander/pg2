use pg2::{deserialize_followers, establish_connection_pool, insert_follower};
use std::{error::Error, time::Duration, time::Instant};
use tokio::runtime;

fn main() -> Result<(), Box<dyn Error>> {
    let runtime = runtime::Builder::new_multi_thread().build()?;

    let followers = deserialize_followers()?;

    println!("inserting {} followers!", followers.len());

    let pool = establish_connection_pool();

    // begin timer
    let start = Instant::now();

    // 200_000 total followers
    if followers.len() != 200_000 {
        panic!("expected 200_000 followers!");
    }

    for fl in followers {
        let pool2 = pool.clone();
        runtime.spawn_blocking(move || {
            let conn = pool2.get().unwrap();
            insert_follower(&conn, fl);
        });
    }

    runtime.shutdown_timeout(Duration::from_secs(120));

    println!("inserting took {:#?}", start.elapsed());

    Ok(())
}
