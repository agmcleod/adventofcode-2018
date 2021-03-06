extern crate read_input;

use std::cmp::{self, Ord, Ordering};
use std::collections::{HashMap, HashSet};

type Coord = (usize, usize);

fn reading_order(a: &Coord, b: &Coord) -> Ordering {
    match a.1.cmp(&b.1) {
        Ordering::Equal => a.0.cmp(&b.0),
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TileType {
    Unpassable,
    Open,
    Unit,
}

#[derive(Copy, Clone, Debug)]
enum UnitType {
    Elf,
    Goblin,
}

#[derive(Copy, Clone, Debug)]
struct Unit {
    unit_type: UnitType,
    hp: i32,
    damage: i32,
    took_turn: bool,
}

impl Unit {
    fn new(unit_type: UnitType, damage: i32) -> Self {
        Unit {
            unit_type,
            hp: 200,
            damage,
            took_turn: false,
        }
    }
}

struct SelectionData {
    target_coord: Coord,
    hp: i32,
    move_to_coord: Coord,
    target_unit_coord: Coord,
}

struct FindNextData {
    scanned_coords: HashSet<Coord>,
    path: Vec<Coord>,
}

impl FindNextData {
    fn new(scanned_coords: HashSet<Coord>, path: Vec<Coord>) -> Self {
        FindNextData {
            scanned_coords,
            path,
        }
    }

    fn get_coord(&self) -> &Coord {
        self.path.get(self.path.len() - 1).unwrap()
    }
}

impl SelectionData {
    fn new(
        target_coord: &Coord,
        hp: i32,
        move_to_coord: &Coord,
        target_unit_coord: &Coord,
    ) -> Self {
        SelectionData {
            target_coord: target_coord.clone(),
            hp,
            move_to_coord: move_to_coord.clone(),
            target_unit_coord: target_unit_coord.clone(),
        }
    }
}

fn hash_map_coords_to_vec<T>(map: &HashMap<Coord, T>) -> Vec<Coord> {
    map.iter().map(|(coord, _)| coord).cloned().collect()
}

fn sort_coords(coords: &mut Vec<Coord>) {
    coords.sort_by(|a, b| match a.1.cmp(&b.1) {
        Ordering::Equal => a.0.cmp(&b.0),
        Ordering::Greater => Ordering::Greater,
        Ordering::Less => Ordering::Less,
    })
}

pub fn get_neighbours(
    scanned_coords: &HashSet<Coord>,
    pos: &Coord,
    tiles: &Vec<Vec<TileType>>,
) -> Vec<(usize, usize, TileType)> {
    let mut neighbours: Vec<(usize, usize, TileType)> = Vec::with_capacity(4);

    // we push coords in reading order
    if pos.1 > 0 && !scanned_coords.contains(&(pos.0, pos.1 - 1)) {
        let tile_type = &tiles[pos.1 - 1][pos.0];
        if *tile_type == TileType::Open || *tile_type == TileType::Unit {
            neighbours.push((pos.0, pos.1 - 1, tile_type.clone()));
        }
    }

    if pos.0 > 0 && !scanned_coords.contains(&(pos.0 - 1, pos.1)) {
        let tile_type = &tiles[pos.1][pos.0 - 1];
        if *tile_type == TileType::Open || *tile_type == TileType::Unit {
            neighbours.push((pos.0 - 1, pos.1, tile_type.clone()));
        }
    }

    if pos.0 < tiles[0].len() - 1 && !scanned_coords.contains(&(pos.0 + 1, pos.1)) {
        let tile_type = &tiles[pos.1][pos.0 + 1];
        if *tile_type == TileType::Open || *tile_type == TileType::Unit {
            neighbours.push((pos.0 + 1, pos.1, tile_type.clone()));
        }
    }

    if pos.1 < tiles.len() - 1 && !scanned_coords.contains(&(pos.0, pos.1 + 1)) {
        let tile_type = &tiles[pos.1 + 1][pos.0];
        if *tile_type == TileType::Open || *tile_type == TileType::Unit {
            neighbours.push((pos.0, pos.1 + 1, tile_type.clone()));
        }
    }

    neighbours
}

fn find_paths(tiles: &Vec<Vec<TileType>>, coord: &Coord, target: &Coord) -> Vec<Vec<Coord>> {
    let mut scanned_coords = HashSet::new();
    scanned_coords.insert(coord.clone());

    let mut paths = Vec::new();

    let mut stack = vec![FindNextData::new(
        scanned_coords.clone(),
        vec![coord.clone()],
    )];

    let mut min_path_length = 10_000;

    while stack.len() > 0 {
        let current = stack.remove(0);
        if current.get_coord() == target {
            min_path_length = cmp::min(min_path_length, current.path.len());
            paths.push(current.path.clone());
            continue;
        }

        if current.path.len() > min_path_length {
            break;
        }

        let neighbours = get_neighbours(&scanned_coords, current.get_coord(), tiles);
        for neighbour in &neighbours {
            scanned_coords.insert((neighbour.0, neighbour.1));
        }

        for neighbour in &neighbours {
            if neighbour.2 == TileType::Unit {
                continue;
            }
            let neighbour = (neighbour.0, neighbour.1);
            let mut path = current.path.clone();
            path.push(neighbour);
            stack.push(FindNextData::new(current.scanned_coords.clone(), path));
        }
    }

    paths
}

fn update_distance_data(
    distance_data: &mut SelectionData,
    new_coord: &Coord,
    target_hp: i32,
    move_to_spot: &Coord,
) {
    let mut current = distance_data.target_coord;
    current.0 = new_coord.0;
    current.1 = new_coord.1;
    distance_data.hp = target_hp;
    distance_data.move_to_coord.0 = move_to_spot.0;
    distance_data.move_to_coord.1 = move_to_spot.1;
}

fn update_target_if_in_reading_order(
    distance_data: &mut SelectionData,
    new_coord: &Coord,
    target_hp: i32,
    move_to_spot: &Coord,
) {
    let target_coord = distance_data.target_coord;
    if new_coord.1 < target_coord.1
        || (new_coord.1 == target_coord.1 && new_coord.0 < target_coord.0)
    {
        update_distance_data(distance_data, new_coord, target_hp, move_to_spot);
    }
}

fn sort_attack_targets(attack_targets: &mut Vec<Coord>, target_units: &HashMap<Coord, Unit>) {
    attack_targets.sort_by(|a, b| {
        let target_a = target_units.get(a).unwrap();
        let target_b = target_units.get(b).unwrap();

        match target_a.hp.cmp(&target_b.hp) {
            Ordering::Equal => reading_order(a, b),
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    });
}

fn get_shortest_path<'a>(paths: &'a mut Vec<Vec<Coord>>) -> Option<&'a Vec<Coord>> {
    paths.sort_by(|a, b| {
        // find shortest paths
        match a.len().cmp(&b.len()) {
            Ordering::Equal => {
                // sort by comparing each coordinate
                let first_coord_a = a.get(0).unwrap();
                let first_coord_b = b.get(0).unwrap();
                if first_coord_a != first_coord_b {
                    return if first_coord_a.1 < first_coord_b.1
                        || (first_coord_a.1 == first_coord_b.1 && first_coord_a.0 < first_coord_b.0)
                    {
                        Ordering::Less
                    } else {
                        Ordering::Greater
                    };
                }
                Ordering::Equal
            }
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        }
    });

    paths.get(0)
}

fn select_target(
    tiles: &Vec<Vec<TileType>>,
    coord: &Coord,
    target_coord: &Coord,
    target_unit: &Unit,
    target_unit_coord: &Coord,
    min_distance: &mut usize,
    distances: &mut HashMap<usize, SelectionData>,
) {
    let mut paths = find_paths(&tiles, coord, target_coord);

    let path = get_shortest_path(&mut paths);

    if path.is_none() {
        return;
    }

    let path = path.unwrap();

    let distance = path.len();
    // 0 index is current spot, get the next one
    let move_to_spot = *path.get(1).unwrap();
    *min_distance = cmp::min(*min_distance, distance);
    if distances.contains_key(&distance) {
        let selection_data = distances.get_mut(&distance).unwrap();
        update_target_if_in_reading_order(
            selection_data,
            &target_coord,
            target_unit.hp,
            &move_to_spot,
        );
    } else {
        distances.insert(
            path.len(),
            SelectionData::new(
                target_coord,
                target_unit.hp,
                &move_to_spot,
                target_unit_coord,
            ),
        );
    }
}

fn attack(
    tiles: &mut Vec<Vec<TileType>>,
    target_units: &mut HashMap<Coord, Unit>,
    target_unit_coord: &Coord,
    actioner: &Unit,
) -> bool {
    let dead = {
        let target_unit = target_units.get_mut(target_unit_coord).unwrap();
        target_unit.hp -= actioner.damage;
        target_unit.hp <= 0
    };
    if dead {
        // println!("Removing unit from {:?}", target_unit_coord);
        *tiles
            .get_mut(target_unit_coord.1)
            .unwrap()
            .get_mut(target_unit_coord.0)
            .unwrap() = TileType::Open;
        target_units.remove(target_unit_coord);
    }

    dead
}

fn perform_move(
    tiles: &mut Vec<Vec<TileType>>,
    empty_map: &HashSet<Coord>,
    units: &mut HashMap<Coord, Unit>,
    actioner_coord: &Coord,
    min_distance: &usize,
    target_units: &mut HashMap<Coord, Unit>,
    distances: &HashMap<usize, SelectionData>,
) -> bool {
    if !distances.contains_key(&min_distance) {
        return false;
    }
    let distance_data = distances.get(min_distance).unwrap();
    let mut unit_died = false;
    // if about to move into spot next to an enemy, do an attack
    if *min_distance <= 2 {
        let mut attack_targets: Vec<Coord> =
            get_neighbours(empty_map, &distance_data.move_to_coord, tiles)
                .iter()
                .filter(|(x, y, tile_type)| {
                    if *tile_type == TileType::Unit {
                        target_units.contains_key(&(*x, *y))
                    } else {
                        false
                    }
                })
                .map(|(x, y, _)| (*x, *y))
                .collect();

        sort_attack_targets(&mut attack_targets, target_units);
        let attack_target = attack_targets.get(0).unwrap();
        unit_died = attack(
            tiles,
            target_units,
            attack_target,
            units.get(actioner_coord).unwrap(),
        );
    }
    // make old spot open, and new one unpassable
    *tiles
        .get_mut(actioner_coord.1)
        .unwrap()
        .get_mut(actioner_coord.0)
        .unwrap() = TileType::Open;
    let move_to_coord = distance_data.move_to_coord;
    *tiles
        .get_mut(move_to_coord.1)
        .unwrap()
        .get_mut(move_to_coord.0)
        .unwrap() = TileType::Unit;

    let actioner: Unit = units.get(actioner_coord).unwrap().to_owned();
    units.insert(move_to_coord.clone(), actioner);
    units.remove(actioner_coord).unwrap();

    unit_died
}

fn print_tiles(
    tiles: &Vec<Vec<TileType>>,
    goblins: &HashMap<(usize, usize), Unit>,
    elves: &HashMap<(usize, usize), Unit>,
) {
    for (y, row) in tiles.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            match col {
                TileType::Open => print!("."),
                TileType::Unpassable => print!("#"),
                TileType::Unit => {
                    if goblins.contains_key(&(x, y)) {
                        print!("G");
                    } else if elves.contains_key(&(x, y)) {
                        print!("E");
                    } else {
                        print!("U");
                    }
                }
            }
        }
        print!("\n");
    }
    print!("\n");

    for (coord, goblin) in goblins {
        println!("{:?} - {:?}", coord, goblin);
    }

    for (coord, elf) in elves {
        println!("{:?} - {:?}", coord, elf);
    }
}

fn take_turn(
    empty_map: &HashSet<Coord>,
    tiles: &mut Vec<Vec<TileType>>,
    unit_collection: &mut HashMap<Coord, Unit>,
    targets: &mut HashMap<Coord, Unit>,
    unit_coord: &Coord,
    min_distance: &mut usize,
    distances: &mut HashMap<usize, SelectionData>,
) -> bool {
    let mut attack_targets = Vec::new();
    unit_collection.get_mut(unit_coord).unwrap().took_turn = true;
    for (target_coord, target) in targets.iter_mut() {
        if target.hp == 0 {
            continue;
        }

        let neighbours = get_neighbours(&empty_map, target_coord, &tiles);
        for neighbour in &neighbours {
            // if unit is next to a target ATTACK!!!!!!!!!! ⚔️
            if (neighbour.0, neighbour.1) == *unit_coord && neighbour.2 == TileType::Unit {
                attack_targets.push(target_coord.clone());
                break;
            }

            // no need to do expensive path finding
            if attack_targets.len() > 0 {
                continue;
            }

            if neighbour.2 == TileType::Open {
                select_target(
                    &tiles,
                    unit_coord,
                    &(neighbour.0, neighbour.1),
                    target,
                    target_coord,
                    min_distance,
                    distances,
                );
            }
        }
    }

    let mut unit_died = false;

    if attack_targets.len() > 0 {
        sort_attack_targets(&mut attack_targets, targets);

        let attack_target = attack_targets.get(0).unwrap();
        unit_died = attack(
            tiles,
            targets,
            attack_target,
            unit_collection.get(unit_coord).unwrap(),
        );
    } else {
        perform_move(
            tiles,
            empty_map,
            unit_collection,
            unit_coord,
            &min_distance,
            targets,
            &distances,
        );
    }

    unit_died
}

fn did_elves_win(
    elves: &HashMap<Coord, Unit>,
    goblins: &HashMap<Coord, Unit>,
    rounds: i32,
) -> bool {
    if goblins.len() == 0 {
        let hp_sum = elves.iter().fold(0, |sum, (_, elf)| sum + elf.hp);
        println!("Elves win {} * {} = {}", rounds, hp_sum, rounds * hp_sum);
        return true;
    }
    false
}

fn did_goblins_win(
    elves: &HashMap<Coord, Unit>,
    goblins: &HashMap<Coord, Unit>,
    rounds: i32,
) -> bool {
    if elves.len() == 0 {
        let hp_sum = goblins.iter().fold(0, |sum, (_, goblin)| sum + goblin.hp);
        println!("Goblins win {} * {} = {}", rounds, hp_sum, rounds * hp_sum);
        return true;
    }
    false
}

fn run_game(text: &String, elf_damage: i32, elves_must_win: bool) -> bool {
    let mut tiles: Vec<Vec<TileType>> = Vec::new();

    let mut goblins = HashMap::new();
    let mut elves = HashMap::new();

    for (y, line) in text.lines().enumerate() {
        let mut row = Vec::new();
        for (x, ch) in line.chars().enumerate() {
            row.push(match ch {
                '.' => TileType::Open,
                '#' => TileType::Unpassable,
                'G' | 'E' => TileType::Unit,
                _ => panic!("Unknown type '{}'", ch),
            });

            if ch == 'G' {
                goblins.insert((x, y), Unit::new(UnitType::Goblin, 3));
            } else if ch == 'E' {
                elves.insert((x, y), Unit::new(UnitType::Elf, elf_damage));
            }
        }

        tiles.push(row);
    }

    let mut rounds = 0;

    // print_tiles(&tiles, &goblins, &elves);

    // used for get_neighbours, for available spots around a target
    let empty_map: HashSet<Coord> = HashSet::new();
    let mut elves_won = false;

    'main: loop {
        if did_elves_win(&elves, &goblins, rounds) {
            elves_won = true;
            break;
        } else if did_goblins_win(&elves, &goblins, rounds) {
            break;
        }

        // these two loops update the took turn flag
        for (_, elf) in &mut elves {
            elf.took_turn = false;
        }

        for (_, goblin) in &mut goblins {
            goblin.took_turn = false;
        }

        let mut sorted_unit_coords = hash_map_coords_to_vec(&goblins);
        let mut sorted_elf_coords = hash_map_coords_to_vec(&elves);

        sorted_unit_coords.append(&mut sorted_elf_coords);

        sort_coords(&mut sorted_unit_coords);

        for coord in &sorted_unit_coords {
            let mut distances: HashMap<usize, SelectionData> = HashMap::new();
            let mut min_distance = 1000;
            if goblins.contains_key(coord) {
                // if the goblin at this coord already took turn (due to a move), skip
                if goblins.get(coord).unwrap().took_turn {
                    continue;
                }
                if did_goblins_win(&elves, &goblins, rounds) {
                    break 'main;
                }
                let unit_died = take_turn(
                    &empty_map,
                    &mut tiles,
                    &mut goblins,
                    &mut elves,
                    &coord,
                    &mut min_distance,
                    &mut distances,
                );

                if unit_died && elves_must_win {
                    return false;
                }
            } else if elves.contains_key(coord) {
                // if the elve at this coord already took turn (due to a move), skip
                if elves.get(coord).unwrap().took_turn {
                    continue;
                }
                if did_elves_win(&elves, &goblins, rounds) {
                    elves_won = true;
                    break 'main;
                }
                take_turn(
                    &empty_map,
                    &mut tiles,
                    &mut elves,
                    &mut goblins,
                    &coord,
                    &mut min_distance,
                    &mut distances,
                );
            }
        }

        // println!("{}", rounds + 1);
        // print_tiles(&tiles, &goblins, &elves);

        rounds += 1;
    }

    elves_won
}

fn main() {
    let text = read_input::read_text("15/input.txt").unwrap();
    run_game(&text, 3, false);

    let mut damage_rate = 20;
    let mut damage = damage_rate;
    let mut last_damage = 0;
    let mut last_outcome_loss = true;
    loop {
        println!("Trying {}", damage);
        if run_game(&text, damage, true) {
            if last_damage + 1 == damage && last_outcome_loss {
                break;
            }
            last_outcome_loss = false;
            // try half way between last two damage points
            damage = (damage - last_damage) / 2 + last_damage;
            // next damage rate should be half of current
            damage_rate = (damage - last_damage) / 2 / 2;
        } else {
            last_damage = damage;
            damage += damage_rate;
            last_outcome_loss = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_generation() {
        let tiles = vec![
            vec![
                TileType::Unpassable,
                TileType::Unpassable,
                TileType::Unpassable,
                TileType::Unpassable,
                TileType::Unpassable,
                TileType::Unpassable,
                TileType::Unpassable,
            ],
            vec![
                TileType::Unpassable,
                TileType::Open,
                TileType::Unit,
                TileType::Open,
                TileType::Open,
                TileType::Open,
                TileType::Unpassable,
            ],
            vec![
                TileType::Unpassable,
                TileType::Open,
                TileType::Open,
                TileType::Open,
                TileType::Unit,
                TileType::Unit,
                TileType::Unpassable,
            ],
            vec![
                TileType::Unpassable,
                TileType::Open,
                TileType::Unpassable,
                TileType::Open,
                TileType::Unpassable,
                TileType::Unit,
                TileType::Unpassable,
            ],
        ];

        let mut paths = find_paths(&tiles, &(2, 1), &(4, 1));

        let path = get_shortest_path(&mut paths).unwrap();
        assert_eq!(*path, vec![(2, 1), (3, 1), (4, 1)]);
    }
}
