pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::{pg::PgConnection, Connection, RunQueryDsl};
// use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use models::{Tweet, Follower};
use std::{env, io::Read, error::Error};

pub const NUM_TWEETS: i64 = 1_000_000;
pub const NUM_FOLLOWERS: i64 = 200_000;
pub const NUM_USERS: i64 = 10_000;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn establish_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().max_size(10).build(manager).unwrap()
}

pub fn insert_tweet(conn: &PgConnection, tw: Tweet) -> () {
    use schema::tweet;

    diesel::insert_into(tweet::table)
        .values(&tw)
        .execute(conn)
        .expect("Error inserting tweet");
}

pub fn read_tweets_from_file() -> Result<Vec<Tweet>, Box<dyn Error>> {
    let mut file = std::fs::File::open("tweets.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let tweets: Vec<Tweet> = serde_json::from_str(&contents)?;
    Ok(tweets)
}

pub fn insert_follower(conn: &PgConnection, fl: Follower) {
    
}