use read_input;

use std::collections::{HashMap, HashSet};

type Coord = (i32, i32, i32, i32);

#[derive(Debug)]
struct Star {
    connected: Vec<Coord>,
}

fn distance(coord_one: &Coord, coord_two: &Coord) -> i32 {
    let x_diff = (coord_one.0 - coord_two.0).abs();
    let y_diff = (coord_one.1 - coord_two.1).abs();
    let z_diff = (coord_one.2 - coord_two.2).abs();
    let w_diff = (coord_one.3 - coord_two.3).abs();

    x_diff + y_diff + z_diff + w_diff
}

fn main() {
    let text = read_input::read_text("25/input.txt").unwrap();

    let coords: Vec<Coord> = text.lines().map(|line| {
        let nums: Vec<i32> = line.split(",").map(|n| n.parse().unwrap()).collect();
        (
            nums[0],
            nums[1],
            nums[2],
            nums[3],
        )
    }).collect();

    let mut stars = HashMap::new();
    for coord in &coords {
        stars.insert(*coord, Star{ connected: Vec::new() });
    }

    for (i, star) in coords.iter().enumerate() {
        for other_star in coords.iter().skip(i + 1) {
            if distance(&star, &other_star) <= 3 {
                stars.get_mut(star).unwrap().connected.push(*other_star);
                stars.get_mut(other_star).unwrap().connected.push(*star);
            }
        }
    }

    // println!("{:?}", stars);

    let mut scanned_list = HashSet::new();
    let mut constellation_count = 0;
    for (coord, star) in &stars {
        if scanned_list.contains(coord) {
            continue
        }

        constellation_count += 1;

        let mut working_nodes: Vec<Coord> = star.connected.iter().filter(|c| !scanned_list.contains(*c)).cloned().collect();

        while let Some(connected_coord) = working_nodes.pop() {
            scanned_list.insert(connected_coord);
            for coord in stars.get(&connected_coord).unwrap().connected.iter().filter(|c| !scanned_list.contains(*c)) {
                working_nodes.push(*coord);
            }
        }
    }

    println!("{}", constellation_count)
}
