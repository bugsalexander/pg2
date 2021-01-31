-- This file should undo anything in `up.sql`
ALTER TABLE tweet
  ALTER COLUMN tweet_text DROP NOT NULL;