-- This file should undo anything in `up.sql`
ALTER TABLE IF EXISTS tweet
  ALTER COLUMN tweet_text DROP NOT NULL;