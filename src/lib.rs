pub mod models;
pub mod schema;

#[macro_use]
extern crate diesel;
extern crate dotenv;

use chrono::NaiveDateTime;
use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
    sql_query, RunQueryDsl,
};

use dotenv::dotenv;
use models::*;
use r2d2_redis::{r2d2, redis::Commands, RedisConnectionManager};
use std::{collections::HashSet, env, error::Error, io::Read, time::Duration};

pub const NUM_TWEETS: i64 = 1_000_000;
pub const NUM_FOLLOWERS: i64 = 200_000;
pub const NUM_USERS: i64 = 10_000;
pub const FILE_TWEETS: &'static str = "tweets.json";
pub const FILE_FOLLOWERS: &'static str = "followers.json";

pub fn establish_postgres_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .max_size(10)
        .connection_timeout(Duration::from_secs(60))
        .build(manager)
        .unwrap()
}

pub fn establish_redis_pool() -> Pool<RedisConnectionManager> {
    dotenv().ok();

    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set in ENV");
    let manager = RedisConnectionManager::new(redis_url)
        .expect("could not find redis, please double check connection url");
    Pool::builder().build(manager).unwrap()
}

// todo: the following functions are pretty much the same. how can they be merged into one?
pub fn insert_tweet(conn: &PgConnection, tw: Tweet) -> () {
    use schema::tweet;

    diesel::insert_into(tweet::table)
        .values(&tw)
        .execute(conn)
        .expect("Error inserting tweet");
}

pub fn insert_redis_tweet_1(conn: &mut r2d2::PooledConnection<RedisConnectionManager>, tw: Tweet) {
    // assume we are inserting in order of timestamp
    // most recent tweets will be at the front of the list
    // map from user id to list of the tweet ids they have tweeted (in order)
    let _: i32 = conn.lpush(user_key(tw.user_id), tw.tweet_id).unwrap();
    // map from tweet id to tweet timestamp and text
    let _1: bool = conn
        .set(
            tweet_key(tw.tweet_id),
            format_tweet(tw.tweet_ts, tw.tweet_text),
        )
        .unwrap();
}

pub fn insert_redis_tweet_2(conn: &mut r2d2::PooledConnection<RedisConnectionManager>, tw: Tweet) {
    // assume we are inserting in order of timestamp
    // most recent tweets will be at the front of the list

    // for each one of the people that follow, push to their home timeline
    let users: HashSet<i64> = conn.smembers(follower_key(tw.user_id)).unwrap();

    for follower_id in users.into_iter() {
        let _: i32 = conn.lpush(user_key(follower_id), tw.tweet_id).unwrap();
    }

    // map from tweet id to tweet timestamp and text
    let _1: bool = conn
        .set(
            tweet_key(tw.tweet_id),
            format_tweet(tw.tweet_ts, tw.tweet_text),
        )
        .unwrap();
}

// kv modeling strategy 1
// TWEET: t:tweet_id -> timestamp|text
// TWEETER: u:uid -> [tweet_id]
// FOLLOWERS: f:uid -> [uid]

// kv modeling strategy 2
// TWEET: t:tweet_id -> timestamp|text
// TWEETER: f:uid -> [uid] (from user id to their followers)
// FOLLOWERS: u:uid -> [tweet_id] (from user id to their timeline, sorted)

pub fn insert_follower(conn: &PgConnection, fl: Follower) {
    use schema::follower;

    diesel::insert_into(follower::table)
        .values(fl)
        .execute(conn)
        .expect("Error inserting follower");
}

pub fn insert_redis_follower_1(
    conn: &mut r2d2::PooledConnection<RedisConnectionManager>,
    fl: Follower,
) {
    let _: i32 = conn
        .sadd(follower_key(fl.user_id), fl.follows_id)
        .expect("sadd failed!");
}

pub fn insert_redis_follower_2(
    conn: &mut r2d2::PooledConnection<RedisConnectionManager>,
    fl: Follower,
) {
    // this time, map from tweeter id -> set<follower id>
    // so when we broadcast, we broadcast tweet -> follower timelines
    let _: i32 = conn
        .sadd(follower_key(fl.follows_id), fl.user_id)
        .expect("sadd failed!");
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

pub fn query_redis_timeline_1(
    conn: &mut r2d2::PooledConnection<RedisConnectionManager>,
    user_id: i64,
) -> Vec<RedisTweet> {
    // get all the uids the person follows
    // for each uid, query last 10 tweets
    // sort all the tweets by timestamp
    // return the most recent 10
    let mems: HashSet<i64> = conn.smembers(follower_key(user_id)).unwrap();
    let mut tweet_ids: Vec<i64> = Vec::new();
    for uid in mems.into_iter() {
        // grab the last 10 tweets
        let mut tids: Vec<i64> = conn.lrange(user_key(uid), 0, 9).unwrap();
        tweet_ids.append(&mut tids);
    }

    // grab all the tweets
    let mut tweets: Vec<RedisTweet> = Vec::new();
    for tid in tweet_ids {
        let tweet_str: String = conn.get(tweet_key(tid)).unwrap();
        tweets.push(parse_redis_tweet(tweet_str));
    }

    // sort them, and return the last 10
    tweets.sort_by(|a, b| b.tweet_ts.cmp(&a.tweet_ts));
    tweets.truncate(10);
    tweets
}

pub fn query_redis_timeline_2(
    conn: &mut r2d2::PooledConnection<RedisConnectionManager>,
    user_id: i64,
) -> Vec<RedisTweet> {
    // get all the tweets from the person, limit 10
    let mut tids: Vec<i64> = conn.lrange(user_key(user_id), 0, 9).unwrap();
    tids.truncate(10);

    let mut tweets = Vec::new();
    for tid in tids {
        let tweet_str: String = conn.get(tweet_key(tid)).unwrap();
        tweets.push(parse_redis_tweet(tweet_str));
    }

    tweets
}

fn follower_key(uid: i64) -> String {
    format!("f:{}", uid)
}

fn tweet_key(tid: i64) -> String {
    format!("t:{}", tid)
}

fn user_key(uid: i64) -> String {
    format!("u:{}", uid)
}

fn format_tweet(tweet_ts: NaiveDateTime, tweet_text: String) -> String {
    format!("{}|{}", tweet_ts, tweet_text)
}

fn parse_redis_tweet(tweet_str: String) -> RedisTweet {
    let mut parts = tweet_str.split("|");

    return RedisTweet {
        tweet_ts: NaiveDateTime::parse_from_str(parts.next().unwrap(), "%Y-%m-%d %H:%M:%S")
            .unwrap(),
        tweet_text: parts.next().unwrap().to_string(),
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono;
    use std::iter::FromIterator;

    #[test]
    fn test_redis_1() {
        let pool = establish_redis_pool();
        let mut pool2 = pool.get().unwrap();

        insert_redis_follower_1(
            &mut pool2,
            Follower {
                user_id: 1,
                follows_id: 5,
            },
        );
        insert_redis_follower_1(
            &mut pool2,
            Follower {
                user_id: 2,
                follows_id: 5,
            },
        );
        insert_redis_follower_1(
            &mut pool2,
            Follower {
                user_id: 3,
                follows_id: 5,
            },
        );

        // users 1, 2, 3 follow 5
        let m_1: HashSet<i64> = pool2.smembers(follower_key(1)).unwrap();
        assert_eq!(m_1.into_iter().collect::<Vec<i64>>(), vec![5]);

        let m_2: HashSet<i64> = pool2.smembers(follower_key(2)).unwrap();
        assert_eq!(m_2.into_iter().collect::<Vec<i64>>(), vec![5]);

        let m_3: HashSet<i64> = pool2.smembers(follower_key(3)).unwrap();
        assert_eq!(m_3.into_iter().collect::<Vec<i64>>(), vec![5]);

        insert_redis_tweet_1(
            &mut pool2,
            Tweet {
                user_id: 5,
                tweet_id: 1000,
                tweet_text: String::from("i love redis"),
                tweet_ts: chrono::NaiveDate::from_ymd(2020, 02, 12).and_hms(0, 53, 53),
            },
        );

        insert_redis_tweet_1(
            &mut pool2,
            Tweet {
                user_id: 5,
                tweet_id: 999,
                tweet_text: String::from("ooga booga"),
                tweet_ts: chrono::NaiveDate::from_ymd(2779, 12, 12).and_hms(0, 53, 53),
            },
        );

        assert_eq!(
            query_redis_timeline_1(&mut pool2, 1),
            [
                RedisTweet {
                    tweet_text: String::from("ooga booga"),
                    tweet_ts: chrono::NaiveDate::from_ymd(2779, 12, 12).and_hms(0, 53, 53),
                },
                RedisTweet {
                    tweet_text: String::from("i love redis"),
                    tweet_ts: chrono::NaiveDate::from_ymd(2020, 02, 12).and_hms(0, 53, 53),
                }
            ]
        );
    }

    #[test]
    fn test_redis_2() {
        let pool = establish_redis_pool();
        let mut pool2 = pool.get().unwrap();

        insert_redis_follower_2(
            &mut pool2,
            Follower {
                user_id: 1,
                follows_id: 5,
            },
        );
        insert_redis_follower_2(
            &mut pool2,
            Follower {
                user_id: 2,
                follows_id: 5,
            },
        );
        insert_redis_follower_2(
            &mut pool2,
            Follower {
                user_id: 3,
                follows_id: 5,
            },
        );

        // users 1, 2, 3 follow 5
        let m_1: HashSet<i64> = pool2.smembers(follower_key(5)).unwrap();
        let m_2: HashSet<i64> = HashSet::from_iter(vec![1i64, 2, 3].iter().cloned());
        assert_eq!(m_1, m_2);

        insert_redis_tweet_2(
            &mut pool2,
            Tweet {
                user_id: 5,
                tweet_id: 1000,
                tweet_text: String::from("i love redis"),
                tweet_ts: chrono::NaiveDate::from_ymd(2020, 02, 12).and_hms(0, 53, 53),
            },
        );

        insert_redis_tweet_2(
            &mut pool2,
            Tweet {
                user_id: 5,
                tweet_id: 999,
                tweet_text: String::from("ooga booga"),
                tweet_ts: chrono::NaiveDate::from_ymd(2779, 12, 12).and_hms(0, 53, 53),
            },
        );

        assert_eq!(
            query_redis_timeline_2(&mut pool2, 1),
            [
                RedisTweet {
                    tweet_text: String::from("ooga booga"),
                    tweet_ts: chrono::NaiveDate::from_ymd(2779, 12, 12).and_hms(0, 53, 53),
                },
                RedisTweet {
                    tweet_text: String::from("i love redis"),
                    tweet_ts: chrono::NaiveDate::from_ymd(2020, 02, 12).and_hms(0, 53, 53),
                }
            ]
        );
    }
}
