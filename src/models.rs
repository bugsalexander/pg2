use super::schema::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "tweet"]
pub struct Tweet {
    pub tweet_id: i64,
    pub tweet_ts: NaiveDateTime,
    pub user_id: i64,
    pub tweet_text: String,
}

#[derive(Debug)]
pub struct RedisTweet {
    pub tweet_ts: NaiveDateTime,
    pub tweet_text: String,
}

#[derive(Debug, Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "follower"]
pub struct Follower {
    pub user_id: i64,
    pub follows_id: i64,
}
