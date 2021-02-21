use pg2::{
    establish_postgres_pool, establish_redis_pool, query_redis_timeline_1, query_timeline,
    NUM_USERS,
};
use rand::{prelude::ThreadRng, thread_rng, Rng};
use std::{env, error::Error, time::Instant};
use std::{
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::runtime::{self, Runtime};

fn main() -> Result<(), Box<dyn Error>> {
    let runtime = runtime::Builder::new_multi_thread().build()?;
    let args: Vec<String> = env::args().collect();
    println!("querying timelines!");

    // begin timer and rng and counter
    let counter = Arc::new(AtomicU32::new(0));
    let mut rng = thread_rng();
    let start = Instant::now();

    if let Some(_mode) = args.get(1) {
        let mode: &str = _mode;
        match mode {
            "postgres" => query_postgres_timelines(&runtime, &mut rng, &counter),
            "redis" => query_redis_timelines_1(&runtime, &mut rng, &counter),
            _ => panic!("please provide either \"postgres\" or \"redis as mode argument\""),
        }
    } else {
        panic!("please provide either \"postgres\" or \"redis as mode argument\"");
    }

    runtime.shutdown_timeout(Duration::from_secs(180));

    println!(
        "queried a total of {:#?} timelines in {:#?}!",
        counter,
        Instant::now() - start
    );

    Ok(())
}

fn query_postgres_timelines(runtime: &Runtime, rng: &mut ThreadRng, counter: &Arc<AtomicU32>) {
    let pool = establish_postgres_pool();

    // generate 10,000 of them, and see how long it takes to finish
    for _i in 0..NUM_USERS {
        let pool2 = pool.clone();
        let uid = rng.gen_range(0..NUM_USERS);
        let counter_ref = counter.clone();
        runtime.spawn_blocking(move || {
            let conn = pool2.get().unwrap();
            query_timeline(&conn, uid);
            counter_ref.fetch_add(1, Ordering::Relaxed);
        });
    }
}

fn query_redis_timelines_1(runtime: &Runtime, rng: &mut ThreadRng, counter: &Arc<AtomicU32>) {
    let pool = establish_redis_pool();

    // generate 10,000 of them, and see how long it takes to finish
    for _i in 0..NUM_USERS {
        let pool2 = pool.clone();
        let uid = rng.gen_range(0..NUM_USERS);
        let counter_ref = counter.clone();
        runtime.spawn_blocking(move || {
            let mut conn = pool2.get().unwrap();
            query_redis_timeline_1(&mut conn, uid);
            counter_ref.fetch_add(1, Ordering::Relaxed);
        });
    }
}
