pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use self::models::{NewPost, Post};
use diesel::{pg::PgConnection, Connection, RunQueryDsl};
// use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use models::Tweet;
use std::env;

pub const NUM_TWEETS: i64 = 1_000_000;
pub const NUM_USERS: i64 = 200_000;

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

pub fn create_post<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> Post {
    use schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn insert_tweet<'a>(conn: &PgConnection, tw: Tweet) -> () {
    use schema::tweet;

    diesel::insert_into(tweet::table)
        .values(&tw)
        .execute(conn)
        .expect("Error inserting tweet");
}
