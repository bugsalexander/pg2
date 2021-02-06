table! {
    follower (user_id, follows_id) {
        user_id -> Int8,
        follows_id -> Int8,
    }
}

table! {
    follower_typeorm (user_id, follows_id) {
        user_id -> Int8,
        follows_id -> Int8,
    }
}

table! {
    tweet (tweet_id) {
        tweet_id -> Int8,
        user_id -> Int8,
        tweet_ts -> Timestamp,
        tweet_text -> Varchar,
    }
}

table! {
    tweet_typeorm (tweet_id) {
        tweet_ts -> Timestamp,
        tweet_id -> Int8,
        user_id -> Int8,
        tweet_text -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    follower,
    follower_typeorm,
    tweet,
    tweet_typeorm,
);
