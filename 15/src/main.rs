extern crate read_input;

use std::collections::HashMap;
use std::cmp::{self, Ordering};

mod astar;

type Coord = (usize, usize);

#[derive(Copy, Clone, Debug)]
enum UnitType {
    Elf,
    Goblin,
}

#[derive(Copy, Clone, Debug)]
struct Unit {
    unit_type: UnitType,
    move_target: Option<Coord>,
    hp: i32,
    damage: i32,
}

impl Unit {
    fn new(unit_type: UnitType) -> Self {
        Unit{
            unit_type,
            move_target: None,
            hp: 200,
            damage: 3,
        }
    }
}

fn hash_map_coords_to_vec<T>(map: &HashMap<Coord, T>) -> Vec<Coord> {
    map.iter().map(|(coord, _)| {
        coord
    })
    .cloned()
    .collect()
}

fn sort_coords(coords: &mut Vec<Coord>) {
    coords.sort_by(|a, b| {
        match a.1.cmp(&b.1) {
            Ordering::Equal => {
                a.0.cmp(&b.0)
            },
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        }
    })
}

fn perform_action(tiles: &mut Vec<Vec<astar::TileType>>, action_units: &mut HashMap<Coord, Unit>, coord: &Coord, target_coord: &Coord, target_unit: &mut Unit, min_distance: &mut usize, distances: &mut HashMap<usize, Coord>) -> bool {
    let path = astar::find_path(&tiles, *coord, *target_coord);
    let distance = path.len();
    *min_distance = cmp::min(*min_distance, distance);
    let mut dead = false;
    if distances.contains_key(&distance) {
        let target = distances.get_mut(&distance).unwrap();
        // if the elf is before the current one in reading order,
        // update the target for the distance
        if target_coord.0 < target.0 || target_coord.1 < target.1 {
            target.0 = target_coord.0;
            target.1 = target_coord.1;
        }
    } else {
        distances.insert(path.len(), target_coord.clone());
    }

    let target = distances.get(&min_distance).unwrap();
    if *min_distance > 0 {
        // we check for 3, as the path includes start & end
        if *min_distance <= 3 {
            let actioner = action_units.get(coord).unwrap();
            target_unit.hp -= actioner.damage;
            if target_unit.hp <= 0 {
                dead = true;
            }
        } else {
            // make old spot open, and new one unpassable
            *tiles.get_mut(coord.0).unwrap().get_mut(coord.1).unwrap() = astar::TileType::Open;
            *tiles.get_mut(target.0).unwrap().get_mut(target.1).unwrap() = astar::TileType::Unpassable;

            let actioner: Unit = action_units.get(coord).unwrap().to_owned();
            action_units.insert(*path.get(path.len() - 2).unwrap(), actioner);
            action_units.remove(coord).unwrap();
        }
    }

    dead
}

fn main() {
    let text = read_input::read_text("15/input.txt").unwrap();
    let mut tiles: Vec<Vec<astar::TileType>> = Vec::new();

    let mut goblins = HashMap::new();
    let mut elves = HashMap::new();

    for (y, line) in text.lines().enumerate() {
        let mut row = Vec::new();
        for (x, ch) in line.chars().enumerate() {
            row.push(
                match ch {
                    '.' => astar::TileType::Open,
                    '#' | 'G' | 'E' => astar::TileType::Unpassable,
                    _ => panic!("Unknown type '{}'", ch),
                }
            );

            if ch == 'G' {
                goblins.insert((x, y), Unit::new(UnitType::Goblin));
            } else if ch == 'E' {
                elves.insert((x, y), Unit::new(UnitType::Elf));
            }
        }

        tiles.push(row);
    }

    let mut combat_rounds = 0;

    loop {
        let mut sorted_unit_coords = hash_map_coords_to_vec(&goblins);
        let mut sorted_elf_coords = hash_map_coords_to_vec(&elves);

        if sorted_unit_coords.len() == 0 {
            println!("Elves win {}", combat_rounds * sorted_elf_coords.iter().fold(0, |sum, coord| {
                sum + elves.get(coord).unwrap().hp
            }));
            break
        } else if sorted_elf_coords.len() == 0 {
            println!("Goblins win {}", combat_rounds * sorted_unit_coords.iter().fold(0, |sum, coord| {
                sum + goblins.get(coord).unwrap().hp
            }));
            break
        }

        sorted_unit_coords.append(&mut sorted_elf_coords);

        sort_coords(&mut sorted_unit_coords);

        for coord in &sorted_unit_coords {
            let mut distances: HashMap<usize, (usize, usize)> = HashMap::new();
            let mut min_distance = 1000;
            let mut dead_coord = None;
            if goblins.contains_key(coord) {
                for (elf_coord, elf) in &mut elves {
                    let dead = perform_action(
                        &mut tiles,
                        &mut goblins,
                        coord,
                        elf_coord,
                        elf,
                        &mut min_distance,
                        &mut distances,
                    );
                    if dead {
                        dead_coord = Some(elf_coord);
                    }
                }
            } else if elves.contains_key(coord) {
                for (goblin_coord, goblin) in &mut goblins {
                    let dead = perform_action(
                        &mut tiles,
                        &mut elves,
                        coord,
                        goblin_coord,
                        goblin,
                        &mut min_distance,
                        &mut distances,
                    );

                    if dead {
                        dead_coord = Some(goblin_coord);
                    }
                }
            }

            if let Some(dead_coord) = dead_coord {
                // we remove from both, as only one can contain that coord
                // and the borrow checker didnt descope mutable borrow after for loop
                elves.remove(dead_coord);
                goblins.remove(dead_coord);
            }
        }
    }
}
