use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use chrono::{DateTime, Local};
use std::time::{SystemTime, UNIX_EPOCH};
use std::time::Duration;
use std::fs::File;
use std::io::Write;

fn get_roll(seed: u64) -> i32 {
    let mut rng: SmallRng = SmallRng::seed_from_u64(seed);
    rng.gen_range(1..101)
}

fn main() {
    let mut arr: Vec<String> = Vec::new();

    let time: SystemTime = SystemTime::now();
    println!("time:");
    println!("{:?}", time);

    let mut seed: u64 = time
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    while arr.len() < 600 {
        let new_seed = seed + 1;
        let roll = get_roll(new_seed);
        let old_time: DateTime<Local> = (UNIX_EPOCH + Duration::from_secs(seed)).into();
        let formatted_time = old_time.format("%H:%M:%S").to_string();
        let entry = format!("{} {}", formatted_time, roll);
        arr.push(entry);
        seed += 1;
    }

    // Write the array to a file
    let mut file = File::create("output.txt").unwrap();
    for line in &arr {
        file.write_all(line.as_bytes()).unwrap();
        file.write_all(b"\n").unwrap();
    }
}
