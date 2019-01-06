use std::collections::HashMap;

const DEPTH: usize = 3558;
const TARGET: (usize, usize) = (15, 740);
const MODULO: usize = 20183;

enum TileType {
    Rocky,
    Wet,
    Narrow
}

impl TileType {
    fn from_num(num: usize) -> TileType {
        match num {
            0 => TileType::Rocky,
            1 => TileType::Wet,
            2 => TileType::Narrow,
            _ => panic!("Invalid number"),
        }
    }
}

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
    let mut grid = HashMap::new();
    let mut erosion_levels = HashMap::new();

    let mut total = 0;
    for x in 0..=TARGET.0 {
        for y in 0..=TARGET.1 {
            let level = erosion_level(&erosion_levels, x, y);
            erosion_levels.insert((x, y), level);

            let modulo = level % 3;
            let tile_type = TileType::from_num(modulo);
            total += modulo;
            grid.insert((x, y), tile_type);
        }
    }

    println!("{}", total);

    // for y in 0..=TARGET.0 {
    //     for x in 0..=TARGET.1 {
    //         match grid.get(&(x, y)).unwrap() {
    //             &TileType::Rocky => print!("."),
    //             &TileType::Wet => print!("="),
    //             &TileType::Narrow => print!("|"),
    //         }
    //     }
    //     print!("\n");
    // }
}
