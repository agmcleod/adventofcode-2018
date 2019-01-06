use std::collections::HashMap;

const DEPTH: usize = 510;
const TARGET: (usize, usize) = (10, 10);
const MODULO: usize = 20183;

mod astar;
mod tile_type;
mod tool;

use crate::tile_type::TileType;

fn erosion_level(erosion_levels: &HashMap<(usize, usize), usize>, x: usize, y: usize) -> usize {
    if (x == 0 && y == 0) || (x == TARGET.0 && y == TARGET.1) {
        DEPTH % MODULO
    } else if x == 0 {
        (y * 48271 + DEPTH) % MODULO
    } else if y == 0 {
        (x * 16807 + DEPTH) % MODULO
    } else {
        (*erosion_levels.get(&(x - 1, y)).unwrap() * *erosion_levels.get(&(x, y - 1)).unwrap() + DEPTH) % MODULO
    }
}

fn main() {
    let mut tiles: Vec<Vec<TileType>> = Vec::new();
    let mut erosion_levels = HashMap::new();

    let mut total = 0;
    for x in 0..=(TARGET.0 + 10) {
        for y in 0..=(TARGET.1 + 10) {
            let level = erosion_level(&erosion_levels, x, y);
            erosion_levels.insert((x, y), level);

            let modulo = level % 3;
            let tile_type = TileType::from_num(modulo);
            if x <= TARGET.0 && y <= TARGET.1 {
                total += modulo;
            }

            if tiles.len() <= y {
                tiles.push(vec![tile_type]);
            } else {
                tiles.get_mut(y).unwrap().push(tile_type);
            }
        }
    }

    println!("{}", total);

    let (path, cost) = astar::find_path(&tiles, (0, 0), TARGET);
    println!("{}", cost);

    for y in 0..=(TARGET.1 + 10) {
        for x in 0..=(TARGET.0 + 10) {
            if path.contains(&(x, y)) {
                print!("o");
            } else {
                match tiles[y][x] {
                    TileType::Rocky => print!("."),
                    TileType::Wet => print!("="),
                    TileType::Narrow => print!("|"),
                }
            }
        }
        print!("\n");
    }
}
