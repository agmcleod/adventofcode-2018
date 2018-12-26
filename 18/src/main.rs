extern crate read_input;

use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {
    Open,
    Trees,
    LumberYard,
}

fn get_adjacents(grid: &HashMap<(i32, i32), Tile>, coord: &(i32, i32)) -> Vec<Tile> {
    let mut adjacents = Vec::new();

    for x in -1..=1 {
        for y in -1..=1 {
            if (x != 0 || y != 0) && grid.contains_key(&(coord.0 + x, coord.1 + y)) {
                let tile_type = grid.get(&(coord.0 + x, coord.1 + y)).unwrap();
                adjacents.push(tile_type.to_owned());
            }
        }
    }

    adjacents
}

fn flatten_state(grid: &HashMap<(i32, i32), Tile>, size: i32) -> String {
    let mut state = String::new();
    for y in 0..size {
        for x in 0..size {
            match grid.get(&(x, y)).unwrap() {
                Tile::Open => state.push('.'),
                Tile::Trees => state.push('|'),
                Tile::LumberYard => state.push('#'),
            }
        }
    }

    state
}

fn main() {
    let text = read_input::read_text("18/input.txt").unwrap();

    let mut grid = HashMap::new();
    let mut states_to_index = HashMap::new();
    let mut list_of_states = Vec::new();

    for (y, line) in text.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let tile_type = match ch {
                '.' => Tile::Open,
                '|' => Tile::Trees,
                '#' => Tile::LumberYard,
                _ => panic!("Unknown tile '{}'", ch),
            };

            grid.insert((x as i32, y as i32), tile_type);
        }
    }

    for i in 0..1_000_000_000 {
        let mut next_state = HashMap::new();
        for (coord, tile_type) in &grid {
            let adjacents = get_adjacents(&grid, coord);
            match tile_type {
                Tile::Open => {
                    if adjacents.iter().filter(|ttype| **ttype == Tile::Trees).count() >= 3 {
                        next_state.insert(coord.clone(), Tile::Trees);
                    } else {
                        next_state.insert(coord.clone(), Tile::Open);
                    }
                },
                Tile::Trees => {
                    if adjacents.iter().filter(|ttype| **ttype == Tile::LumberYard).count() >= 3 {
                        next_state.insert(coord.clone(), Tile::LumberYard);
                    } else {
                        next_state.insert(coord.clone(), Tile::Trees);
                    }
                },
                Tile::LumberYard => {
                    let mut lumber_count = 0;
                    let mut trees_count = 0;
                    for tile_type in &adjacents {
                        if *tile_type == Tile::Trees {
                            trees_count += 1;
                        } else if *tile_type == Tile::LumberYard {
                            lumber_count += 1;
                        }
                    }

                    if lumber_count > 0 && trees_count > 0 {
                        next_state.insert(coord.clone(), Tile::LumberYard);
                    } else {
                        next_state.insert(coord.clone(), Tile::Open);
                    }
                },
            }
        }

        grid = next_state;

        let flattened_state = flatten_state(&grid, 50);
        if states_to_index.contains_key(&flattened_state) {
            let index = states_to_index.get(&flattened_state).unwrap();
            // some math i ended up trying, with a mix of things i read on reddit.
            // subtracting an extra 1 from the original loop index to account for zero indexing.
            let state: &String = list_of_states.get(((1_000_000_000 - index - 1) % (i - index) + index) as usize).unwrap();
            let mut lumber_count = 0;
            let mut trees_count = 0;

            for ch in state.chars() {
                if ch == '|' {
                    trees_count += 1;
                } else if ch == '#' {
                    lumber_count += 1;
                }
            }
            println!("Part two: {} * {} = {}", trees_count, lumber_count, trees_count * lumber_count);
            break
        } else if i == 9 {
            let mut lumber_count = 0;
            let mut trees_count = 0;

            for (_, tile_type) in &grid {
                if *tile_type == Tile::Trees {
                    trees_count += 1;
                } else if *tile_type == Tile::LumberYard {
                    lumber_count += 1;
                }
            }

            println!("Part one: {} * {} = {}", trees_count, lumber_count, trees_count * lumber_count);
        }

        states_to_index.insert(flattened_state.clone(), i);
        list_of_states.push(flattened_state);
    }
}
