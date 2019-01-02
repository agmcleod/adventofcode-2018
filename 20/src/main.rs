use read_input;

use std::collections::{HashMap, HashSet};
use std::cmp;

type Coord = (i32, i32);

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West
}

#[derive(Clone)]
struct Room {
    doors: HashSet<Direction>,
    coord: Coord,
}

impl Room {
    fn new(coord: Coord) -> Self {
        Room{
            doors: HashSet::new(),
            coord,
        }
    }
}

fn modify_coord_by_direction(coord: &mut Coord, direction: &Direction) {
    match *direction {
        Direction::North => {
            coord.1 -= 1;
        },
        Direction::East => {
            coord.0 += 1;
        },
        Direction::South => {
            coord.1 += 1;
        },
        Direction::West => {
            coord.0 -= 1;
        },
    }
}

fn get_direction_from_char(ch: char) -> Option<Direction> {
    match ch {
        'N' => Some(Direction::North),
        'E' => Some(Direction::East),
        'S' => Some(Direction::South),
        'W' => Some(Direction::West),
        _ => None,
    }
}

fn get_opposite(direction: &Direction) -> Direction {
    match *direction {
        Direction::North => Direction::South,
        Direction::East => Direction::West,
        Direction::South => Direction::North,
        Direction::West => Direction::East,
    }
}

fn draw_map(map: &HashMap<Coord, Room>, min_x: i32, min_y: i32, max_x: i32, max_y: i32) {
    for y in min_y..=max_y {
        // above room line
        for x in min_x..=max_x {
            print!("#");
            if let Some(room) = map.get(&(x, y)) {
                if room.doors.contains(&Direction::North) {
                    print!("-");
                } else {
                    print!("#");
                }
            } else {
                print!("#");
            }
        }

        // room line
        print!("\n");
        for x in min_x..=max_x {
            if let Some(room) = map.get(&(x, y)) {
                if room.doors.contains(&Direction::West) {
                    print!("|");
                } else {
                    print!("#");
                }
                print!(".");
            } else {
                print!("##");
            }
        }
        print!("\n");
    }
}

fn setup_map(text: &String, map: &mut HashMap<Coord, Room>, min_x: &mut i32, min_y: &mut i32, max_x: &mut i32, max_y: &mut i32) {
    let mut coord: Coord = (0, 0);
    map.insert(coord, Room::new(coord));
    let mut branching_coords: Vec<Coord> = Vec::new();
    for (i, ch) in text.chars().enumerate() {
        if ch == '(' {
            branching_coords.push(coord);
        } else if ch == '|' {
            // not doing bounds check here, as | shouldnt be last character
            let next_ch = text.chars().nth(i + 1).unwrap();
            // dont reset to branch coord, if next ends this group
            if next_ch != ')' {
                let last_branch = branching_coords.get(branching_coords.len() - 1).unwrap();
                coord.0 = last_branch.0;
                coord.1 = last_branch.1;
            }
        } else if ch == ')' {
            let last_branch = branching_coords.pop().unwrap();
            coord.0 = last_branch.0;
            coord.1 = last_branch.1;
        } else {
            if let Some(direction) = get_direction_from_char(ch) {
                // add the direction to the doors
                let room = map.get_mut(&coord).unwrap();
                room.doors.insert(direction.clone());
                modify_coord_by_direction(&mut coord, &direction);

                // insert next room if it doesnt exist
                if !map.contains_key(&coord) {
                    map.insert(coord, Room::new(coord));
                }

                // add the door we just came through
                let direction = get_opposite(&direction);
                let room = map.get_mut(&coord).unwrap();
                room.doors.insert(direction);

                *min_x = cmp::min(*min_x, coord.0);
                *max_x = cmp::max(*max_x, coord.0);

                *min_y = cmp::min(*min_y, coord.1);
                *max_y = cmp::max(*max_y, coord.1);
            }
        }
    }
}

fn get_neighbours(traversed_rooms: &HashSet<Coord>, room: &Room) -> Vec<Coord> {
    let mut neighbours = Vec::new();

    for door in &room.doors {
        let mut coord = room.coord.clone();
        modify_coord_by_direction(&mut coord, door);
        if !traversed_rooms.contains(&coord) {
            neighbours.push(coord);
        }
    }

    neighbours
}

fn main() {
    let text = read_input::read_text("20/input.txt").unwrap();
    let text = text.replace("^", "").replace("$", "");

    let mut map: HashMap<Coord, Room> = HashMap::new();

    let mut min_x = 0;
    let mut min_y = 0;

    let mut max_x = 0;
    let mut max_y = 0;

    setup_map(&text, &mut map, &mut min_x, &mut min_y, &mut max_x, &mut max_y);
    // draw_map(&map, min_x, min_y, max_x, max_y);

    let mut traversed_rooms: HashSet<Coord> = HashSet::new();
    traversed_rooms.insert((0, 0));

    // this might be unnecessary. Looks like one can track the paths when they parse the text.
    // ah well
    let mut working_stack = vec![((0, 0), vec![(0, 0)])];
    let mut full_paths = Vec::new();
    while working_stack.len() > 0 {
        let (coord, path) = working_stack.remove(0);
        let room = map.get(&coord).unwrap();
        traversed_rooms.insert(coord);
        let neighbours = get_neighbours(&traversed_rooms, room);

        if neighbours.len() == 0 {
            full_paths.push(path);
            continue
        }

        for neighbour in &neighbours {
            let mut path = path.clone();
            path.push(*neighbour);
            working_stack.push((*neighbour, path));
        }
    }

    full_paths.sort_by(|a, b| {
        b.len().cmp(&a.len())
    });

    println!("{}", full_paths.get(0).unwrap().len() - 1);

    let mut past_1k = HashSet::new();
    // println!("{}", full_paths.iter().fold(0, |sum, path| {
    //     if path.len() >= 1000 {
    //         sum + 1
    //     } else {
    //         sum
    //     }
    // }));

    for path in &full_paths {
        if path.len() > 1000 {
            for coord in path.iter().skip(1000) {
                past_1k.insert(*coord);
            }
        }
    }

    println!("{}", past_1k.len());
}
