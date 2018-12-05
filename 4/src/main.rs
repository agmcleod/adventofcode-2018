extern crate read_input;

use std::cmp;
use std::collections::HashMap;

#[derive(Debug)]
enum Mode {
    StartsShift,
    WakesUp,
    FallsAsleep,
}

#[derive(Debug)]
struct Entry {
    guard_id: usize,
    date: String,
    time: usize,
    mode: Mode,
}

#[derive(Debug)]
struct SleepEntry {
    times: HashMap<usize, usize>,
    total_count: usize,
}

impl SleepEntry {
    fn new() -> Self {
        SleepEntry {
            times: HashMap::new(),
            total_count: 0,
        }
    }
}

fn sol(entries: &Vec<Entry>) {
    let mut sleep_tracker = HashMap::new();
    let mut guard_id = 0;
    let mut sleep_time = 0;
    let mut highest_sleep_time = 0;
    for entry in entries {
        if !sleep_tracker.contains_key(&entry.guard_id) {
            sleep_tracker.insert(entry.guard_id, SleepEntry::new());
        }

        match entry.mode {
            Mode::StartsShift => {
                guard_id = entry.guard_id;
            }
            Mode::FallsAsleep => {
                sleep_time = entry.time;
            }
            Mode::WakesUp => {
                for minute in sleep_time..entry.time {
                    let mut sleep_entry = sleep_tracker.get_mut(&entry.guard_id).unwrap();
                    if sleep_entry.times.contains_key(&minute) {
                        *sleep_entry.times.get_mut(&minute).unwrap() += 1;
                    } else {
                        sleep_entry.times.insert(minute, 1);
                    }

                    sleep_entry.total_count += 1;
                    highest_sleep_time = cmp::max(highest_sleep_time, sleep_entry.total_count);
                }
            }
        }
    }

    let mut flattened_minute_data = Vec::new();
    for (guard_id, sleep_entry) in &sleep_tracker {
        if sleep_entry.total_count == highest_sleep_time {
            // (minute, how many times) for that guard
            let mut max_sleep = (0, 0);
            for (minute, count) in &sleep_entry.times {
                if max_sleep.1 < *count {
                    max_sleep.0 = *minute;
                    max_sleep.1 = *count;
                }

                flattened_minute_data.push((minute, count, guard_id));
            }
            println!(
                "guard {} slept on {} {} times, meaning {}",
                guard_id,
                max_sleep.0,
                max_sleep.1,
                max_sleep.0 * guard_id
            );
        } else {
            for (minute, count) in &sleep_entry.times {
                flattened_minute_data.push((minute, count, guard_id));
            }
        }
    }

    flattened_minute_data.sort_by(|a, b| b.1.cmp(a.1));

    println!("{:?}", flattened_minute_data[0]);

    println!(
        "{}",
        flattened_minute_data[0].0 * flattened_minute_data[0].2
    );
}

fn main() {
    let text = read_input::read_text("4/input.txt").unwrap();

    let mut lines: Vec<&str> = text.lines().collect();
    lines.sort();

    let mut entries = Vec::with_capacity(lines.len());

    let mut guard_id = 0;
    for line in &lines {
        let mut pieces = line.split("] ");
        let first_half = pieces.next().unwrap();
        let timestamp = first_half.replace("[", "");
        let mut timestamp = timestamp.split(" ");
        let date = timestamp.next().unwrap().to_string();
        let time = timestamp.next().unwrap();
        let time = time.split(":")
            .nth(1)
            .unwrap()
            .parse()
            .map_err(|err| {
                panic!("Could not parse minutes from {} - error: {}", line, err);
            })
            .unwrap();

        let command = pieces.next().unwrap();

        let mode = match command {
            "falls asleep" => Mode::FallsAsleep,
            "wakes up" => Mode::WakesUp,
            _ => {
                guard_id = command
                    .replace("Guard #", "")
                    .split(" ")
                    .next()
                    .unwrap()
                    .parse()
                    .map_err(|err| {
                        panic!("Could not parse number from {} - error: {}", command, err);
                    })
                    .unwrap();
                Mode::StartsShift
            }
        };

        let mut entry = Entry {
            guard_id,
            date,
            time,
            mode,
        };

        entries.push(entry);
    }

    sol(&entries);
}
