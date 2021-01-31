-- Your SQL goes here
CREATE TABLE IF NOT EXISTS tweet (
  tweet_id BIGINT NOT NULL PRIMARY KEY,
  user_id BIGINT NOT NULL,
  tweet_ts TIMESTAMP NOT NULL,
  tweet_text VARCHAR(140)
);