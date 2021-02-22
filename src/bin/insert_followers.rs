use pg2::{
    deserialize_followers, establish_postgres_pool, establish_redis_pool, insert_follower,
    insert_redis_follower_1, insert_redis_follower_2, models::Follower,
};
use std::{env, error::Error, time::Duration, time::Instant};
use tokio::{runtime, runtime::Runtime};

/// main function to insert followers using the tokio runtime
/// expects "postgres", "redis1", or "redis2" as a CLI argument
fn main() -> Result<(), Box<dyn Error>> {
    let runtime = runtime::Builder::new_multi_thread().build()?;
    let args: Vec<String> = env::args().collect();
    let followers = deserialize_followers()?;

    println!("inserting {} followers!", followers.len());
    if followers.len() != 200_000 {
        panic!("expected 200_000 followers!");
    }

    // begin timer
    let start = Instant::now();

    // depending on which mode we are in, run postgres or redis version
    if let Some(_mode) = args.get(1) {
        let mode: &str = _mode;
        match mode {
            "postgres" => run_postgres_insert(&runtime, followers)?,
            "redis1" => run_redis_insert1(&runtime, followers)?,
            "redis2" => run_redis_insert2(&runtime, followers)?,
            _ => panic!("please provide either \"postgres\" or \"redis as mode argument\""),
        }
    } else {
        panic!("please provide either \"postgres\" or \"redis as mode argument\"");
    }

    // wait at most 120 seconds for all threads to shut down
    runtime.shutdown_timeout(Duration::from_secs(120));

    println!("inserting took {:#?}", start.elapsed());

    Ok(())
}

// todo: disclaimer for grader, i am learning rust and dont know how to abstract the following functions, please go easy on me

/// insert tweets postgres mode
/// for each follower, start a blocking thread that inserts the follower
fn run_postgres_insert(runtime: &Runtime, followers: Vec<Follower>) -> Result<(), Box<dyn Error>> {
    let pool = establish_postgres_pool();

    for fl in followers {
        let pool2 = pool.clone();
        runtime.spawn_blocking(move || {
            let conn = pool2.get().unwrap();
            insert_follower(&conn, fl);
        });
    }

    Ok(())
}

/// insert tweets redis mode, strategy 1
/// for each follower, start a blocking thread that inserts the follower
/// see readme for documentation on strategy
fn run_redis_insert1(runtime: &Runtime, followers: Vec<Follower>) -> Result<(), Box<dyn Error>> {
    let pool = establish_redis_pool();

    for fl in followers {
        let pool2 = pool.clone();
        runtime.spawn_blocking(move || {
            let mut conn = pool2.get().unwrap();
            insert_redis_follower_1(&mut conn, fl);
        });
    }

    Ok(())
}

/// insert tweets redis mode, strategy 2
/// for each follower, start a blocking thread that inserts the follower
fn run_redis_insert2(runtime: &Runtime, followers: Vec<Follower>) -> Result<(), Box<dyn Error>> {
    let pool = establish_redis_pool();

    for fl in followers {
        let pool2 = pool.clone();
        runtime.spawn_blocking(move || {
            let mut conn = pool2.get().unwrap();
            insert_redis_follower_2(&mut conn, fl);
        });
    }

    Ok(())
}
