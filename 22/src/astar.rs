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

fn get_other_tool_type(tile_type: &TileType, current_tool: &Tool) -> Tool {
    match *tile_type {
        TileType::Rocky => {
            if *current_tool == Tool::ClimbingGear {
                Tool::Torch
            } else {
                Tool::ClimbingGear
            }
        },
        TileType::Wet => {
            if *current_tool == Tool::ClimbingGear {
                Tool::Neither
            } else {
                Tool::ClimbingGear
            }
        },
        TileType::Narrow => {
            if *current_tool == Tool::Neither {
                Tool::Torch
            } else {
                Tool::Neither
            }
        }
    }
}

fn get_neighbours(pos: &(usize, usize), current_tool: &Tool, tiles: &Vec<Vec<TileType>>) -> Vec<((usize, usize), Tool)> {
    let mut coords: Vec<(usize, usize)> = Vec::new();

    if pos.0 > 0 {
        coords.push((pos.0 - 1, pos.1));
    }

    if pos.0 < tiles[0].len() - 1 {
        coords.push((pos.0 + 1, pos.1));
    }

    if pos.1 > 0 {
        coords.push((pos.0, pos.1 - 1));
    }

    if pos.1 < tiles.len() - 1 {
        coords.push((pos.0, pos.1 + 1));
    }

    let current_tile_type = tiles[pos.1][pos.0];

    let mut neighbours = Vec::new();
    for coord in &coords {
        let tile_type = tiles[coord.1][coord.0];
        if tile_type == current_tile_type {
            neighbours.push((coord.to_owned(), *current_tool));
            neighbours.push((coord.to_owned(), get_other_tool_type(&tile_type, current_tool)));
        } else {
            let tool = match current_tile_type {
                TileType::Rocky => {
                    match tile_type {
                        TileType::Narrow => Tool::Torch,
                        TileType::Wet => Tool::ClimbingGear,
                        _ => panic!("Cannot change against same tile type")
                    }
                },
                TileType::Wet => {
                    match tile_type {
                        TileType::Rocky => Tool::ClimbingGear,
                        TileType::Narrow => Tool::Neither,
                        _ => panic!("Cannot change against same tile type")
                    }
                },
                TileType::Narrow => {
                    match tile_type {
                        TileType::Rocky => Tool::Torch,
                        TileType::Wet => Tool::Neither,
                        _ => panic!("Cannot change against same tile type")
                    }
                }
            };

            neighbours.push((coord.to_owned(), tool));
        }
    }

    neighbours
}

pub fn find_path(tiles: &Vec<Vec<TileType>>, start_pos: (usize, usize), target: (usize, usize)) -> Vec<(usize, usize)> {
    let mut costs: HashMap<((usize, usize), Tool), usize> = HashMap::new();
    costs.insert((start_pos, Tool::Torch), 0);

    let mut heap = BinaryHeap::new();
    heap.push(Location::new(start_pos, 0, tiles[start_pos.1][start_pos.0], Tool::Torch, 0));

    // current pos, points to last pos + cost of getting here
    let mut closed: HashMap<((usize, usize), Tool), ((usize, usize), Tool)> = HashMap::new();
    closed.insert((start_pos, Tool::Torch), (start_pos, Tool::Torch));

    let mut end_closed = ((0, 0), Tool::Torch);

    while let Some(location) = heap.pop() {
        if location.position.0 == target.0 && location.position.1 == target.1 {
            end_closed.0 = location.position;
            end_closed.1 = location.tool;
            if location.tool != Tool::Torch {
                println!("end add torch {}", location.minutes + 7);
            } else {
                println!("end {}", location.minutes);
            }

            break
        }

        let neighbours = get_neighbours(&location.position, &location.tool, &tiles);
        for (neighbour, tool_type) in neighbours {
            let target_tile_type = tiles[neighbour.1][neighbour.0];
            let mut offset_cost = 1;
            if tool_type != location.tool {
                offset_cost = 8;
            }
            let new_cost = costs.get(&(location.position, location.tool)).unwrap() + offset_cost;
            if !costs.contains_key(&(neighbour, tool_type)) || new_cost < *costs.get(&(neighbour, tool_type)).unwrap() {
                heap.push(
                    Location::new(
                        neighbour,
                        new_cost + distance_to_target(&neighbour, &target),
                        target_tile_type,
                        tool_type,
                        location.minutes + offset_cost
                    )
                );
                costs.insert((neighbour, tool_type), new_cost);
                closed.insert((neighbour, tool_type), (location.position, location.tool));
            }
        }
    }

    let mut path: Vec<(usize, usize)> = Vec::new();

    if closed.contains_key(&end_closed) {
        path.push(target);
        let mut key = end_closed;
        loop {
            let parent_node = closed.get(&key).unwrap();
            // println!("{:?} using {:?}", key.0, parent_node.1);
            if parent_node.0 == key.0 {
                break
            }
            path.push(parent_node.0);
            key = *parent_node;
        }
    }

    path
}
