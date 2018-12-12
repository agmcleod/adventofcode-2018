extern crate read_input;

use std::cmp;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
struct Vector2 {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Spec {
    position: Vector2,
    velocity: Vector2,
}

impl Spec {
    fn new(x: i32, y: i32, velx: i32, vely: i32) -> Self {
        Spec{
            position: Vector2{ x, y },
            velocity: Vector2{ x: velx, y: vely },
        }
    }
}

fn coords_string_to_vec(string: &str) -> Vec<i32> {
    string.split(",").map(|n| {
        if let Ok(result) = n.parse() {
            result
        } else {
            panic!("Could not parse {} as i32", n);
        }
    }).collect()
}

fn add_counts_to_map(map: &mut HashMap<i32, usize>, value: i32) {
    if map.contains_key(&value) {
        *map.get_mut(&value).unwrap() += 1;
    } else {
        map.insert(value, 1);
    }
}

fn main() {
    let text = read_input::read_text("10/input.txt").unwrap();
    let mut specs: Vec<Spec> = text.lines().map(|line| {
        let line = line.replace(" ", "").replace(">", "").replace("position=<", "");
        let mut iter = line.split("velocity=<");
        let pos: Vec<i32> = coords_string_to_vec(iter.next().unwrap());
        let vel: Vec<i32> = coords_string_to_vec(iter.next().unwrap());

        Spec::new(
            *pos.get(0).unwrap(),
            *pos.get(1).unwrap(),
            *vel.get(0).unwrap(),
            *vel.get(1).unwrap(),
        )
    }).collect();

    let spec_count = specs.len() as f32;

    let mut seconds = 0;
    loop {
        let mut x_counts = HashMap::new();
        let mut y_counts = HashMap::new();
        let mut grid = HashSet::new();

        let mut min_x = 100_000;
        let mut min_y = 100_000;
        let mut max_x = -100_000;
        let mut max_y = -100_000;

        seconds += 1;

        for spec in specs.iter_mut() {
            spec.position.x += spec.velocity.x;
            spec.position.y += spec.velocity.y;

            add_counts_to_map(&mut x_counts, spec.position.x);
            add_counts_to_map(&mut y_counts, spec.position.y);

            grid.insert((spec.position.x, spec.position.y));

            min_x = cmp::min(min_x, spec.position.x);
            min_y = cmp::min(min_y, spec.position.y);

            max_x = cmp::max(max_x, spec.position.x);
            max_y = cmp::max(max_y, spec.position.y);
        }

        if (x_counts.len() as f32) < spec_count / 4.0 && (y_counts.len() as f32) < spec_count / 4.0 {
            println!("seconds: {}", seconds);
            for y in min_y..=max_y {
                for x in min_x..=max_x {
                    if grid.contains(&(x, y)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                print!("\n");
            }

            print!("\n\n");
        }
    }
}
