use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::Write,
};

use pg2::{models::Follower, NUM_FOLLOWERS, NUM_USERS};
use rand::{thread_rng, Rng};

pub fn main() -> Result<(), Box<dyn Error>> {
    let mut tracker = HashMap::<i64, HashSet<i64>>::new();
    let mut followers: Vec<Follower> = vec![];

    let mut rng = thread_rng();
    let mut gen_uid = || rng.gen_range(0..NUM_USERS);

    for _i in 0..NUM_FOLLOWERS {
        // generate the user that is going to follow someone
        let user_id = gen_uid();
        // if they don't already exist in the tracker, create an entry
        if !tracker.contains_key(&user_id) {
            tracker.insert(user_id, HashSet::new());
        }

        // generate the id of the person they are following
        // while already following person, or trying to follow themselves, regenerate
        let mut follows_id = gen_uid();
        while tracker.get(&user_id).unwrap().contains(&follows_id) || follows_id == user_id {
            follows_id = gen_uid();
        }
        // add it to the tracker, add to list
        tracker.get_mut(&user_id).unwrap().insert(follows_id);
        followers.push(Follower {
            user_id,
            follows_id,
        });
    }

    // convert to JSON
    let json = serde_json::to_string(&followers).expect("failed to serialize");

    // write to file
    let mut file = File::create("followers.json").expect("file create failed");
    file.write_all(json.as_bytes()).expect("write failed");
    println!("file written!");

    Ok(())
}
