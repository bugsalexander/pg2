use std::io::Write;

use chrono::NaiveDateTime;
use fake::{faker::lorem::en::Sentence, Fake, Faker};
use pg2::{NUM_TWEETS, NUM_USERS, models::Tweet};
use std::fs::File;

pub fn main() {
    /*
    tweet_id: i,
    tweet_ts: faker.date.past(PAST_YEARS),
    user_id: Math.floor(Math.random() * NUM_USERS),
    tweet_text: faker.lorem.words(Math.floor(Math.random() * MAX_WORDS) + 1),
    */
    let mut tweets: Vec<Tweet> = vec![];
    for n in 0..NUM_TWEETS {
        let tw = Tweet {
            tweet_id: n,
            tweet_ts: Faker.fake::<NaiveDateTime>(),
            user_id: (0..NUM_USERS).fake::<i64>(),
            tweet_text: Sentence(1..10).fake(),
        };
        tweets.push(tw);
    }

    // convert to JSON
    let json = serde_json::to_string(&tweets).expect("failed to serialize");

    // write to file
    let mut file = File::create("tweets.json").expect("file create failed");
    file.write_all(json.as_bytes()).expect("write failed");
    println!("file written!");
}
