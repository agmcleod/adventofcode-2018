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

fn update_target_if_in_reading_order(current: &mut (usize, usize), new_coord: &(usize, usize), distance_hp: &mut i32, target_hp: i32) -> bool {
    if new_coord.0 < current.0 || new_coord.1 < current.1 {
        current.0 = new_coord.0;
        current.1 = new_coord.1;
        *distance_hp = target_hp;
        return true
    }

    false
}

fn select_target(tiles: &Vec<Vec<astar::TileType>>, coord: &Coord, target_coord: &Coord, target_unit: &mut Unit, min_distance: &mut usize, distances: &mut HashMap<usize, (Coord, i32, Coord)>) {
    let path = astar::find_path(&tiles, *coord, *target_coord);

    let distance = path.len();
    if distance > 0 {
        let move_to_spot = *path.get(path.len() - 2).unwrap();
        *min_distance = cmp::min(*min_distance, distance);
        if distances.contains_key(&distance) {
            let distance_data = distances.get_mut(&distance).unwrap();
            // if the elf is before the current one in reading order,
            // update the target for the distance
            if distance <= 2 {
                if target_unit.hp < distance_data.1 {
                    (distance_data.0).0 = target_coord.0;
                    (distance_data.0).1 = target_coord.1;
                    distance_data.1 = target_unit.hp;
                } else if target_unit.hp == distance_data.1 {
                    update_target_if_in_reading_order(&mut distance_data.0, &target_coord, &mut distance_data.1, target_unit.hp);
                }
            } else {
                if update_target_if_in_reading_order(&mut distance_data.0, &target_coord, &mut distance_data.1, target_unit.hp) {
                    (distance_data.2).0 = move_to_spot.0;
                    (distance_data.2).1 = move_to_spot.1;
                }
            }
        } else {
            distances.insert(path.len(), (target_coord.clone(), target_unit.hp, move_to_spot));
        }
    }
}

fn perform_action(tiles: &mut Vec<Vec<astar::TileType>>, units: &mut HashMap<Coord, Unit>, target_units: &mut HashMap<Coord, Unit>, coord: &Coord, min_distance: &usize, distances: &HashMap<usize, (Coord, i32, Coord)>) {
    if !distances.contains_key(&min_distance) {
        return
    }
    let distance_data = distances.get(min_distance).unwrap();
    // we check for 2 as the path includes start & end
    if *min_distance <= 2 {
        let actioner = units.get(coord).unwrap();
        let dead = {
            let target_unit = target_units.get_mut(&distance_data.0).unwrap();
            target_unit.hp -= actioner.damage;
            target_unit.hp <= 0
        };
        if dead {
            *tiles.get_mut((distance_data.0).1).unwrap().get_mut((distance_data.0).0).unwrap() = astar::TileType::Open;
            target_units.remove(&distance_data.0);
        }
    } else {
        // make old spot open, and new one unpassable
        println!("Move from {:?} to {:?}", coord, distance_data.2);

        *tiles.get_mut(coord.1).unwrap().get_mut(coord.0).unwrap() = astar::TileType::Open;
        let move_to_coord = distance_data.2;
        *tiles.get_mut(move_to_coord.1).unwrap().get_mut(move_to_coord.0).unwrap() = astar::TileType::Unit;

        let actioner: Unit = units.get(coord).unwrap().to_owned();
        units.insert(move_to_coord.clone(), actioner);
        units.remove(coord).unwrap();
    }
}

fn print_tiles(tiles: &Vec<Vec<astar::TileType>>, goblins: &HashMap<(usize, usize), Unit>, elves: &HashMap<(usize, usize), Unit>) {
    for (y, row) in tiles.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            match col {
                astar::TileType::Open => print!("."),
                astar::TileType::Unpassable => print!("#"),
                astar::TileType::Unit => {
                    if goblins.contains_key(&(x, y)) {
                        print!("G");
                    } else if elves.contains_key(&(x, y)) {
                        print!("E");
                    } else {
                        print!("U");
                    }

                },
            }
        }
        print!("\n");
    }
    print!("\n");
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
                    '#' => astar::TileType::Unpassable,
                    'G' | 'E' => astar::TileType::Unit,
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

    let mut rounds = 0;

    print_tiles(&tiles, &goblins, &elves);

    loop {
        let mut sorted_unit_coords = hash_map_coords_to_vec(&goblins);
        let mut sorted_elf_coords = hash_map_coords_to_vec(&elves);

        if sorted_unit_coords.len() == 0 {
            println!("Elves win {}", rounds * sorted_elf_coords.iter().fold(0, |sum, coord| {
                sum + elves.get(coord).unwrap().hp
            }));
            break
        } else if sorted_elf_coords.len() == 0 {
            let hp_sum = sorted_unit_coords.iter().fold(0, |sum, coord| {
                sum + goblins.get(coord).unwrap().hp
            });
            println!("Goblins win {} * {} = {}", rounds, hp_sum, rounds * hp_sum);
            break
        }

        rounds += 1;

        sorted_unit_coords.append(&mut sorted_elf_coords);

        sort_coords(&mut sorted_unit_coords);

        for coord in &sorted_unit_coords {
            let mut distances: HashMap<usize, (Coord, i32, Coord)> = HashMap::new();
            let mut min_distance = 1000;
            if goblins.contains_key(coord) {
                for (elf_coord, elf) in &mut elves {
                    select_target(
                        &tiles,
                        coord,
                        elf_coord,
                        elf,
                        &mut min_distance,
                        &mut distances,
                    );
                }

                perform_action(&mut tiles, &mut goblins, &mut elves, coord, &min_distance, &distances);
            } else if elves.contains_key(coord) {
                for (goblin_coord, goblin) in &mut goblins {
                    select_target(
                        &tiles,
                        coord,
                        goblin_coord,
                        goblin,
                        &mut min_distance,
                        &mut distances,
                    );
                }

                perform_action(&mut tiles, &mut elves, &mut goblins, coord, &min_distance, &distances);
            }
        }

        println!("Round: {}", rounds);
        print_tiles(&tiles, &goblins, &elves);
    }
}
