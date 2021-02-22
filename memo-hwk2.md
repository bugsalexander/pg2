# Assignment 2: Twitter in Redis

DS 4300 - Prof. Rachlin
Alexander Takayama
Feb 21, 2020

### Modeling Strategy

For strategy 1, I modeled my data in redis in the following manner:

```
// TWEET: t:tweet_id -> timestamp|text
// USER: u:uid -> [tweet_id]
// FOLLOWERS: f:uid -> [uid]

```

One big difference from the last assignment, is that I made the assumption that tweets will be inserted in chronological order. This makes everything much easier, because you can assume, for example, that a user's list of tweets is sorted newest -> oldest. 

For strategy 1, a user's id maps to a list of the tweets they have tweeted (in chronological order, with most recent closer to the front). This means that when we are retrieving tweets, we will only need to look up the first 10 tweets of each of the users they follow, since only the top 10 most recent should be returned. Last, a follower's id maps to a set of the user ids they follow. For this one, I thought it made more sense to use a set, and then just use iterators to grab all the tweets of the users (we don't care about being sorted).

For strategy 2, I used the following data model:

```
// TWEET: t:tweet_id -> timestamp|text
// same as strategy 1
// USER: u:uid -> [tweet_id] (from user id to their timeline, sorted)
// FOLLOWER: f:uid -> [uid] (from user id to their followers)

```

Again, being able to use the assumption that tweets are inserted in chronological order makes our lives much easier. This time, a user's id maps to the tweet ids in their home timeline, again with the most recent at the front. Now retrieving tweets is very fast, because we don't have to look up anything other than the 10 tweets at the front of the list. Likewise, tweeter id now maps from their user id to the user ids of their followers, so that when inserting, we can lookup their followers, and add the tweet id to each of their home timelines.

### Benchmark Results

```
Postgres:
Inserts/second: ~13143.71336
Timelines/second: ~70.0280112

Redis Strategy 1:
Inserts/second: ~33107
Timelines/second: ~927

Redis Strategy 2:
Inserts/second: ~25458
Timelines/second: ~8855
```

I definitely can't complain with this performance, but relative to the rest of the crowd (on the spreadsheet) I was surprised that these results weren't at the top, given that I wrote this benchmark in language that compiles to native binaries. I did notice that when running, there was only one distinct process for redis, whereas for postgres I saw 10 distinct processes (one for each thread in the threadpool). I might look into it a bit more after submitting this homework, but the performance was still quite good.

I think it's cool to see that even on our crappy laptops, Redis can pretty much keep up with actual Twitter speeds (of course discounting all the other infrastructure they have setup, load balancing, replication, etc).

### System Information

I ran my benchmarks on a 13-inch, 2019, Macbook Pro, with all other applications closed. My Mac has the following stats:

```
Processor Name: Quad-Core Intel Core i7
Processor Speed: 2.8 GHz
Number of Processors:	1
Total Number of Cores:	4
Memory:	16 GB
```
