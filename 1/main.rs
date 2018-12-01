extern crate read_input;

use std::collections::HashSet;

fn main() {
    let text = read_input::read_text("1/input.txt").unwrap();

    let nums: Vec<i32> = text.lines().map(|line| line.parse().expect("Could not parse")).collect();
    let sum = nums.iter().fold(0, |n, v| n + v);
    println!("{}", sum);

    let mut set = HashSet::new();
    set.insert(0i32);

    let mut sum = 0i32;
    let mut index = 0;

    loop {
        sum += nums.get(index % nums.len()).unwrap();
        if set.contains(&sum) {
            println!("already seen {}", sum);
            break
        } else {
            set.insert(sum);
            index += 1;
        }
    }
}
