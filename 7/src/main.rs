extern crate read_input;
extern crate regex;

use std::collections::{HashMap, HashSet};

use regex::Regex;

fn part_one(no_parents: &HashSet<String>, tree: &HashMap<String, Vec<String>>, child_to_parents: &HashMap<String, Vec<String>>) {
    let mut tree_path = Vec::new();
    let mut processed_keys = HashSet::new();
    for start_key in no_parents {
        let mut current_keys = vec![start_key.clone()];
        loop {
            let key = current_keys.remove(0);
            if let Some(next_keys) = tree.get(&key) {
                tree_path.push(key.clone());
                processed_keys.insert(key);
                for next_key in next_keys {
                    if let Some(parents) = child_to_parents.get(next_key) {
                        if parents.iter().fold(true, |result, key| result && processed_keys.contains(key)) {
                            current_keys.push(next_key.clone());
                        }
                    }
                }
                current_keys.sort();
            } else {
                tree_path.push(key);
            }

            if current_keys.len() == 0 {
                break
            }
        }
    }

    println!("{}", tree_path.join(""));
}

struct Worker {
    letter: String,
    seconds_remaining: usize,
}

impl Worker {
    fn new() -> Self {
        Worker{
            letter: "".to_string(),
            seconds_remaining: 0,
        }
    }
}

fn part_two(no_parents: &HashSet<String>, tree: &HashMap<String, Vec<String>>, child_to_parents: &HashMap<String, Vec<String>>) {
    let mut seconds_required = HashMap::new();
    let alphabet = (b'A'..=b'Z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<_>>();

    for (i, letter) in alphabet.iter().enumerate() {
        seconds_required.insert(letter.to_string(), 60 + i + 1);
    }

    let mut workers = vec![
        Worker::new(),
        Worker::new(),
        Worker::new(),
        Worker::new(),
        Worker::new(),
    ];

    let mut processed_keys = HashSet::new();
    let mut ticker = 0;
    for start_key in no_parents {
        let mut current_keys = vec![start_key.clone()];
        loop {
            for worker in workers.iter_mut().filter(|worker| worker.seconds_remaining == 0) {
                if current_keys.len() > 0 {
                    let key = current_keys.remove(0);
                    worker.letter = key.clone();
                    worker.seconds_remaining = *seconds_required.get(&worker.letter).unwrap();
                }
            }

            let mut working = false;
            for worker in &mut workers {
                if worker.seconds_remaining > 0 {
                    worker.seconds_remaining -= 1;
                    working = true;
                    if worker.seconds_remaining == 0 {
                        if let Some(next_keys) = tree.get(&worker.letter) {
                            processed_keys.insert(worker.letter.clone());
                            for next_key in next_keys {
                                if let Some(parents) = child_to_parents.get(next_key) {
                                    if parents.iter().fold(true, |result, key| result && processed_keys.contains(key)) {
                                        current_keys.push(next_key.clone());
                                    }
                                }
                            }
                            current_keys.sort();
                        }
                    }
                }
            }

            if !working && current_keys.len() == 0 {
                break
            } else {
                ticker += 1;
            }
        }
    }

    println!("{}", ticker);
}

fn main() {
    let text = read_input::read_text("7/input.txt").unwrap();

    let mut tree: HashMap<String, Vec<String>> = HashMap::new();
    let mut child_to_parents: HashMap<String, Vec<String>> = HashMap::new();
    let mut no_parents = HashSet::new();
    let mut scanned_letters = HashSet::new();

    let re = Regex::new(r"[A-Z]").unwrap();
    for line in text.lines() {
        let cap = re.captures_iter(line);
        let mut cap = cap.skip(1);
        let letter_one = cap.next().unwrap().get(0).unwrap().as_str().to_string();
        let letter_two = cap.next().unwrap().get(0).unwrap().as_str().to_string();

        // since it has a dependency, remove it
        if no_parents.contains(&letter_two) {
            no_parents.remove(&letter_two);
        }

        if tree.contains_key(&letter_one) {
            let mut keys = tree.get_mut(&letter_one).unwrap();
            keys.push(letter_two.clone());
            keys.sort();
        } else {
            tree.insert(letter_one.clone(), vec![letter_two.clone()]);
        }

        if child_to_parents.contains_key(&letter_two) {
            let mut parents = child_to_parents.get_mut(&letter_two).unwrap();
            parents.push(letter_one.clone());
            parents.sort();
        } else {
            child_to_parents.insert(letter_two.clone(), vec![letter_one.clone()]);
        }

        // if we havent come across a dependency yet, it might be a parent
        if !scanned_letters.contains(&letter_one) {
            scanned_letters.insert(letter_one.clone());
            no_parents.insert(letter_one.clone());
        }

        scanned_letters.insert(letter_two.clone());
    }

    if no_parents.len() == 1 {
        part_one(&no_parents, &tree, &child_to_parents);
        part_two(&no_parents, &tree, &child_to_parents);
    } else {
        println!("Something went wrong");
    }
}
