extern crate read_input;

use std::collections::{HashMap};
use std::cmp;

#[derive(Debug)]
struct Vector2 {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct SquareInch {
    ids: Vec<usize>,
}

impl SquareInch {
    fn new() -> Self {
        SquareInch{
            ids: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Claim {
    id: usize,
    pos: Vector2,
    size: Vector2,
}

impl Claim {
    fn from_text(id: usize, text: &str) -> Self {
        let transform = text.split("@ ").nth(1).unwrap();
        let mut pieces = transform.split(": ");
        let mut pos = Vector2{x: 0, y: 0};
        let mut coords = pieces.next().unwrap().split(",");
        pos.x = coords.next().unwrap().parse().expect("Could not parse x");
        pos.y = coords.next().unwrap().parse().expect("Could not parse y");

        let mut size = Vector2{x: 0, y: 0};
        let mut coords = pieces.next().unwrap().split("x");
        size.x = coords.next().unwrap().parse().expect("Could not parse w");
        size.y = coords.next().unwrap().parse().expect("Could not parse h");

        Claim{
            id,
            pos,
            size,
        }
    }
}

fn main() {
    let input = read_input::read_text("input.txt").unwrap();

    let claims: Vec<Claim> = input.lines().enumerate().map(|(i, line)| {
        Claim::from_text(i + 1, line)
    }).collect();

    let mut fabric: HashMap<(usize, usize), SquareInch> = HashMap::new();
    let mut count = 0;
    for (i, claim) in claims.iter().enumerate() {
        let id = i + 1;
        // iterate through the square
        for x in claim.pos.x..(claim.pos.x + claim.size.x) {
            for y in claim.pos.y..(claim.pos.y + claim.size.y) {
                let mut si = if fabric.contains_key(&(x, y)) {
                    fabric.get_mut(&(x, y)).unwrap()
                } else {
                    fabric.insert((x, y), SquareInch::new());
                    fabric.get_mut(&(x, y)).unwrap()
                };

                // store this id in the location
                si.ids.push(id);
                if si.ids.len() == 2 {
                    count += 1;
                }
            }
        }
    }

    println!("{}", count);

    // store the highest count for each id, across their squares.
    // As some can have a count of 1, and others a count of 2 for the same ID
    let result = fabric.iter().fold(HashMap::new(), |mut obj, (_, square_inch)| {
        for id in &square_inch.ids {
            if obj.contains_key(&id) {
                let mut count = obj.get_mut(&id).unwrap();
                *count = cmp::max(*count, square_inch.ids.len());
            } else {
                obj.insert(id, square_inch.ids.len());
            }
        }
        obj
    });

    for (id, value) in &result {
        if *value == 1 {
            println!("{}", id);
        }
    }
}