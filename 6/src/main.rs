extern crate read_input;

use std::cmp;
use std::collections::{HashMap, HashSet};

pub fn get_manhatten_distance(from_coords: &(usize, usize), x: usize, y: usize) -> usize {
    ((from_coords.0 as i32 - x as i32).abs() + (from_coords.1 as i32 - y as i32).abs()) as usize
}

fn main() {
    let text = read_input::read_text("6/input.txt").unwrap();
    let mut coords = HashMap::new();

    let mut min_x = 1_000_000;
    let mut min_y = 1_000_000;
    let mut max_x = 0;
    let mut max_y = 0;

    for line in text.lines() {
        let mut coord = (0, 0);
        let mut iter = line.split(", ").map(|v| v.parse().unwrap());
        coord.0 = iter.next().unwrap();
        coord.1 = iter.next().unwrap();

        min_x = cmp::min(coord.0, min_x);
        min_y = cmp::min(coord.1, min_y);
        max_x = cmp::max(coord.0, max_x);
        max_y = cmp::max(coord.1, max_y);

        coords.insert(coord, 0);
    }

    let mut infinite_coords = HashSet::new();

    let mut reachable_within_10k = 0;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let mut distances = Vec::new();
            for (pair, _) in &(coords) {
                let distance = get_manhatten_distance(pair, x, y);
                distances.push((pair.clone(), distance));
            }

            if distances
                .iter()
                .fold(0, |sum, (_, distance)| sum + distance)
                < 10_000
            {
                reachable_within_10k += 1;
            }

            distances.sort_by(|a, b| a.1.cmp(&b.1));

            if distances[0].1 < distances[1].1 {
                *coords.get_mut(&distances[0].0).unwrap() += 1;
                if x == min_x || x == max_x || y == min_y || y == max_y {
                    infinite_coords.insert(distances[0].0.clone());
                }
            }
        }
    }

    let mut max_count = 0;
    for (coord, count) in &coords {
        if !infinite_coords.contains(coord) {
            max_count = cmp::max(max_count, *count);
        }
    }

    println!("{}", max_count);

    println!("{}", reachable_within_10k);
}

#[test]
fn test_manhattent_distance() {
    assert_eq!(get_manhatten_distance(&(1, 5), 6, 12), 12);
    assert_eq!(get_manhatten_distance(&(3, 3), 15, 1), 14);
}
