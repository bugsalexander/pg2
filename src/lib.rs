pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
    sql_query, RunQueryDsl,
};
use dotenv::dotenv;
use models::*;
use std::{env, error::Error, io::Read};

pub const NUM_TWEETS: i64 = 1_000_000;
pub const NUM_FOLLOWERS: i64 = 200_000;
pub const NUM_USERS: i64 = 10_000;
pub const FILE_TWEETS: &'static str = "tweets.json";
pub const FILE_FOLLOWERS: &'static str = "followers.json";

pub fn establish_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().max_size(10).build(manager).unwrap()
}

// todo: the following functions are pretty much the same. how can they be merged into one?
pub fn insert_tweet(conn: &PgConnection, tw: Tweet) -> () {
    use schema::tweet;

    diesel::insert_into(tweet::table)
        .values(&tw)
        .execute(conn)
        .expect("Error inserting tweet");
}

pub fn insert_follower(conn: &PgConnection, fl: Follower) {
    use schema::follower;

    diesel::insert_into(follower::table)
        .values(fl)
        .execute(conn)
        .expect("Error inserting follower");
}

// todo: the following functions are pretty much the same. how can they be merged into one?
pub fn deserialize_tweets() -> Result<Vec<Tweet>, Box<dyn Error>> {
    let mut file = std::fs::File::open(FILE_TWEETS)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let result: Vec<Tweet> = serde_json::from_str(&contents)?;
    Ok(result)
}

pub fn deserialize_followers() -> Result<Vec<Follower>, Box<dyn Error>> {
    let mut file = std::fs::File::open(FILE_FOLLOWERS)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let result: Vec<Follower> = serde_json::from_str(&contents)?;
    Ok(result)
}

pub fn query_timeline(conn: &PgConnection, user_id: i64) -> () {
    sql_query(format!(
        "\
        select tw.* \
        from follower as fw \
        left join tweet as tw \
        on tw.user_id = fw.follows_id \
        where fw.user_id = {} \
        order by tw.tweet_ts desc \
        limit 10\
        ",
        user_id
    ))
    .execute(conn)
    .expect("Error querying timeline");
}
