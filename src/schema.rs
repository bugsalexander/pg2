table! {
    follower (user_id, follows_id) {
        user_id -> Int8,
        follows_id -> Int8,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
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

allow_tables_to_appear_in_same_query!(
    follower,
    posts,
    tweet,
);
