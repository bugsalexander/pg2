use super::schema::*;
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

/*
tweet_id: i,
    tweet_ts: faker.date.past(PAST_YEARS),
    user_id: Math.floor(Math.random() * NUM_USERS),
    tweet_text: faker.lorem.words(Math.floor(Math.random() * MAX_WORDS) + 1),
*/
#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "tweet"]
pub struct Tweet {
    pub tweet_id: i64,
    pub tweet_ts: NaiveDateTime,
    pub user_id: i64,
    pub tweet_text: String,
}

#[derive(Insertable)]
#[table_name="posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
