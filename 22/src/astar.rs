use std::cmp::Ordering;
use std::collections::{HashMap, BinaryHeap};

use crate::tile_type::TileType;
use crate::tool::Tool;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Location {
    position: (usize, usize),
    estimated_cost: usize,
    tile_type: TileType,
    tool: Tool,
    minutes: usize,
}

impl Location {
    fn new(position: (usize, usize), estimated_cost: usize, tile_type: TileType, tool: Tool, minutes: usize) -> Self {
        Location{
            position,
            estimated_cost,
            tile_type,
            tool,
            minutes,
        }
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Location) -> Ordering {
        other.estimated_cost.cmp(&self.estimated_cost)
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Location) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn distance_to_target(location: &(usize, usize), target: &(usize, usize)) -> usize {
    let mut x_diff = location.1 as i16 - target.1 as i16;
    let mut y_diff = location.0 as i16 - target.0 as i16;
    if x_diff < 0 {
        x_diff *= -1;
    }
    if y_diff < 0 {
        y_diff *= -1;
    }

    x_diff as usize + y_diff as usize
}

pub fn get_neighbours(pos: &(usize, usize), tiles: &Vec<Vec<TileType>>) -> Vec<(usize, usize)> {
    let mut neighbours: Vec<(usize, usize)> = Vec::with_capacity(4);

    if pos.0 > 0 {
        neighbours.push((pos.0 - 1, pos.1));
    }

    if pos.0 < tiles[0].len() - 1 {
        neighbours.push((pos.0 + 1, pos.1));
    }

    if pos.1 > 0 {
        neighbours.push((pos.0, pos.1 - 1));
    }

    if pos.1 < tiles.len() - 1 {
        neighbours.push((pos.0, pos.1 + 1));
    }

    neighbours
}

fn get_next_cost(current_tool: &Tool, current_type: &TileType, target_tile_type: &TileType) -> (usize, Tool) {
    let mut resulting_tool = current_tool.to_owned();

    let cost = match *target_tile_type {
        TileType::Rocky => {
            match current_tool {
                Tool::Neither => {
                    // were in a wet spot, so we use climbing gear
                    if *current_type == TileType::Wet {
                        resulting_tool = Tool::ClimbingGear;
                    // were in a narrow spot, so we use a torch
                    } else if *current_type == TileType::Narrow {
                        resulting_tool = Tool::Torch;
                    }
                    8
                },
                _ => 1,
            }
        },
        TileType::Wet => {
            match current_tool {
                Tool::Torch => {
                    // were in a rocky spot, so we use climbing gear
                    if *current_type == TileType::Rocky {
                        resulting_tool = Tool::ClimbingGear;
                    // were in a narrow spot, so we use a neither
                    } else if *current_type == TileType::Narrow {
                        resulting_tool = Tool::Neither;
                    }
                    8
                },
                _ => 1,
            }
        },
        TileType::Narrow => {
            match current_tool {
                Tool::ClimbingGear => {
                    // were in a rocky spot, so we use torch
                    if *current_type == TileType::Rocky {
                        resulting_tool = Tool::Torch;
                    // were in a wet spot, so we use a neither
                    } else if *current_type == TileType::Wet {
                        resulting_tool = Tool::Neither;
                    }
                    8
                },
                _ => 1,
            }
        },
    };

    (cost, resulting_tool)
}

pub fn find_path(tiles: &Vec<Vec<TileType>>, start_pos: (usize, usize), target: (usize, usize)) -> Vec<(usize, usize)> {
    let mut costs: HashMap<(usize, usize), usize> = HashMap::new();
    costs.insert(start_pos, 0);

    let mut heap = BinaryHeap::new();
    heap.push(Location::new(start_pos, 0, tiles[start_pos.1][start_pos.0], Tool::Torch, 0));

    // current pos, points to last pos + cost of getting here
    let mut closed: HashMap<(usize, usize), ((usize, usize), Tool)> = HashMap::new();
    closed.insert(start_pos, (start_pos, Tool::Torch));

    while let Some(location) = heap.pop() {
        if location.position.0 == target.0 && location.position.1 == target.1 {
            if location.tool != Tool::Torch {
                println!("end add torch {}", location.minutes + 7);
            } else {
                println!("end {}", location.minutes);
            }

            // break
        }
        let neighbours = get_neighbours(&location.position, &tiles);
        for neighbour in neighbours {
            let target_tile_type = tiles[neighbour.1][neighbour.0];
            let current_tile_type = tiles[location.position.1][location.position.0];
            let (offset_cost, tool_type) = get_next_cost(&location.tool, &current_tile_type, &target_tile_type);
            let new_cost = costs.get(&location.position).unwrap() + offset_cost;
            if !costs.contains_key(&neighbour) || new_cost < *costs.get(&neighbour).unwrap() {
                println!("{:?} {:?} using {:?} from {:?}", neighbour, target_tile_type, tool_type, location.position);
                heap.push(
                    Location::new(
                        neighbour,
                        new_cost + distance_to_target(&neighbour, &target),
                        target_tile_type,
                        tool_type,
                        location.minutes + offset_cost
                    )
                );
                costs.insert(neighbour, new_cost);
                closed.insert(neighbour, (location.position, tool_type));
            }
        }
    }

    let mut path: Vec<(usize, usize)> = Vec::new();

    if closed.contains_key(&target) {
        path.push(target);
        let mut key = target;
        loop {
            let (parent, tool_type) = closed.get(&key).unwrap();
            println!("{},{} using {:?}", key.0, key.1, tool_type);
            if *parent == key {
                break
            }
            path.push(*parent);
            key = *parent;
        }
    }

    path
}
