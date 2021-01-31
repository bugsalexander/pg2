-- Your SQL goes here

-- delete tweets with null text
DELETE FROM tweet WHERE tweet_text is NULL;

-- alter table add nonnullable
ALTER TABLE tweet
  ALTER COLUMN tweet_text SET NOT NULL;