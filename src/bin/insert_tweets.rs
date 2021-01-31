use pg2::{establish_connection_pool, insert_tweet, models::Tweet};
use std::{error::Error, fs::File, io::Read, thread};

use tokio::{join, try_join};

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("tweets.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let tweets: Vec<Tweet> = serde_json::from_str(&contents)?;

    println!("inserting {} tweets!", tweets.len());

    let pool = establish_connection_pool();

    // for tw in tweets {
    //     let pool2 = pool.clone();
    //     thread::spawn(move || {
    //         let conn = pool2.get().unwrap();
    //         insert_tweet(&conn, tw)
    //     });
    // }

    let inserts = tweets.into_iter().map(|tw| {
        let pool2 = pool.clone();
        tokio::spawn(async move {
            let conn = pool2.get().unwrap();
            insert_tweet(&conn, tw);
        })
    });

    for handle in inserts {
        try_join!(handle)?;
    }

    Ok(())
}
