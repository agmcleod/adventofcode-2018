extern crate read_input;

use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Down,
    Left,
    Right,
    Up,
}

impl Direction {
    fn from(&mut self, turn: &Turn) {
        *self = match *self {
            Direction::Down => {
                match turn {
                    Turn::Left => Direction::Right,
                    Turn::Right => Direction::Left,
                    Turn::Straight => Direction::Down,
                }
            },
            Direction::Left => {
                match turn {
                    Turn::Left => Direction::Down,
                    Turn::Right => Direction::Up,
                    Turn::Straight => Direction::Left,
                }
            },
            Direction::Right => {
                match turn {
                    Turn::Left => Direction::Up,
                    Turn::Right => Direction::Down,
                    Turn::Straight => Direction::Right,
                }
            },
            Direction::Up => {
                match turn {
                    Turn::Left => Direction::Left,
                    Turn::Right => Direction::Right,
                    Turn::Straight => Direction::Up,
                }
            },
        };
    }
}

#[derive(Copy, Clone, Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn next(&mut self) {
        match *self {
            Turn::Left => {
                *self = Turn::Straight;
            },
            Turn::Straight => {
                *self = Turn::Right;
            },
            Turn::Right => {
                *self = Turn::Left;
            }
        }
    }
}

enum Rail {
    Straight,
    Intersection,
    ForwardSlashCorner,
    BackSlashCorner,
}

#[derive(Copy, Clone, Debug)]
struct Cart {
    direction: Direction,
    turn: Turn,
}

impl Cart {
    fn new(direction: Direction) -> Self {
        Cart{
            direction,
            turn: Turn::Right,
        }
    }
}

fn add_direction_to_position(pos: &mut (usize, usize), direction: Direction) {
    match direction {
        Direction::Down => {
            pos.1 += 1;
        },
        Direction::Left => {
            pos.0 -= 1;
        },
        Direction::Up => {
            pos.1 -= 1;
        },
        Direction::Right => {
            pos.0 += 1;
        },
    }
}

fn step(railways: &HashMap<(usize, usize), Rail>, carts: &mut HashMap<(usize, usize), Cart>) -> Option<(usize, usize)> {
    let mut cart_orders: Vec<((usize, usize))> = carts.iter().map(|(pos, _)| {
        pos.clone()
    }).collect();

    cart_orders.sort_by(|a, b| {
        match a.1.cmp(&(b.1)) {
            Ordering::Equal => {
                a.0.cmp(&(b.0))
            },
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        }
    });

    let mut positions_moved_to = HashSet::new();
    let mut collision_pos = None;

    for pos in &cart_orders {
        let mut cart: Cart = carts.get(&pos).unwrap().to_owned();
        carts.remove(&pos).unwrap();
        let mut next_pos = pos.clone();
        add_direction_to_position(&mut next_pos, cart.direction);
        if positions_moved_to.contains(&next_pos) || carts.contains_key(&next_pos) {
            collision_pos = Some(next_pos);
            break
        } else {
            positions_moved_to.insert(next_pos.clone());

            match railways.get(&next_pos) {
                Some(railway) => {
                    match railway {
                        Rail::ForwardSlashCorner => {
                            cart.direction = match cart.direction {
                                Direction::Down => Direction::Left,
                                Direction::Left => Direction::Down,
                                Direction::Right => Direction::Up,
                                Direction::Up => Direction::Right,
                            }
                        },
                        Rail::BackSlashCorner => {
                            cart.direction = match cart.direction {
                                Direction::Down => Direction::Right,
                                Direction::Left => Direction::Up,
                                Direction::Right => Direction::Down,
                                Direction::Up => Direction::Left,
                            }
                        },
                        Rail::Intersection => {
                            cart.turn.next();
                            cart.direction.from(&cart.turn);
                        },
                        _ => {}
                    }

                    carts.insert(next_pos, cart);
                },
                None => {
                    panic!("Could not find railway at: {:?}. {:?}, current pos: {:?}", next_pos, cart, pos);
                }
            }
        }
    }

    collision_pos
}

fn main() {
    let text = read_input::read_text("13/input.txt").unwrap();

    let mut railways = HashMap::new();
    let mut carts = HashMap::new();

    for (y, line) in text.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let rail = match ch {
                '/' => Rail::ForwardSlashCorner,
                '\\' => Rail::BackSlashCorner,
                '+' => Rail::Intersection,
                '|' | '-' => Rail::Straight,
                ' ' => continue,
                '<' | '>' | '^' | 'v' => {
                    if ch == '<' {
                        carts.insert((x, y), Cart::new(Direction::Left));
                    } else if ch == '>' {
                        carts.insert((x, y), Cart::new(Direction::Right));
                    } else if ch == '^' {
                        carts.insert((x, y), Cart::new(Direction::Up));
                    } else if ch == 'v' {
                        carts.insert((x, y), Cart::new(Direction::Down));
                    }
                    Rail::Straight
                },
                _ => panic!("Could not match '{}'", ch),
            };

            railways.insert((x, y), rail);
        }
    }

    let mut tick = 0;
    loop {
        if let Some(collision) = step(&railways, &mut carts) {
            println!("Collision at: {:?} - {}", collision, tick);
            break
        }
        tick += 1;
    }
}
