use pg2::{establish_postgres_pool, query_timeline};
use rand::{thread_rng, Rng};
use std::{error::Error, time::Instant};
use std::{
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::runtime;

fn main() -> Result<(), Box<dyn Error>> {
    let runtime = runtime::Builder::new_multi_thread().build()?;

    println!("querying timelines!");

    let pool = establish_postgres_pool();

    // begin timer and rng and counter
    let counter = Arc::new(AtomicU32::new(0));
    let mut rng = thread_rng();
    let start = Instant::now();

    // generate 10,000 of them, and see how long it takes to finish
    for _i in 0..10_000 {
        let pool2 = pool.clone();
        let uid = rng.gen_range(0..20_000);
        let counter_ref = counter.clone();
        runtime.spawn_blocking(move || {
            let conn = pool2.get().unwrap();
            query_timeline(&conn, uid);
            counter_ref.fetch_add(1, Ordering::Relaxed);
        });
    }
    
    runtime.shutdown_timeout(Duration::from_secs(180));

    println!(
        "queried a total of {:#?} timelines in {:#?}!",
        counter,
        Instant::now() - start
    );

    Ok(())
}
