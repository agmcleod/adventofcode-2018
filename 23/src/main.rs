use read_input;

use std::cmp;
use std::collections::{BinaryHeap, HashMap, HashSet};

type Coord = (i64, i64, i64);

struct Nanobot {
    pos: Coord,
    radius: i64,
}

#[derive(Debug, Eq, PartialEq)]
struct SubGrid {
    count: i64,
    start: Coord,
    end: Coord,
    depth: i64,
}

impl SubGrid {
    fn new(count: i64, start: Coord, end: Coord, depth: i64) -> Self {
        SubGrid{
            count,
            start,
            end,
            depth,
        }
    }
}

impl cmp::Ord for SubGrid {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        match self.count.cmp(&other.count) {
            cmp::Ordering::Equal => match self.depth.cmp(&other.depth) {
                cmp::Ordering::Equal => {
                    let self_manhattan = (self.start.0).abs() + (self.start.1).abs() + (self.start.2).abs();
                    let other_manhattan = (other.start.0).abs() + (other.start.1).abs() + (other.start.2).abs();
                    other_manhattan.cmp(&self_manhattan)
                },
                n => n,
            },
            n => n,
        }
    }
}

impl PartialOrd for SubGrid {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn distance(location: &Coord, target: &Coord) -> i64 {
    let x_diff = (location.0 - target.0).abs();
    let y_diff = (location.1 - target.1).abs();
    let z_diff = (location.2 - target.2).abs();

    x_diff + y_diff + z_diff
}

fn clamp_to_closest(v: i64, min_v: i64, max_v: i64) -> i64 {
    if v < min_v {
        min_v
    } else if v > max_v {
        max_v
    } else {
        v
    }
}

fn check_rect(
    start: Coord,
    end: Coord,
    nanobots: &Vec<Nanobot>,
    grids: &mut BinaryHeap<SubGrid>,
    depth: i64,
) {
    let mut count = 0;

    for bot in nanobots {
        let x = clamp_to_closest(bot.pos.0, start.0, end.0);
        let y = clamp_to_closest(bot.pos.1, start.1, end.1);
        let z = clamp_to_closest(bot.pos.2, start.2, end.2);

        if distance(&bot.pos, &(x, y, z)) <= bot.radius {
            count += 1;
        }
    }

    // println!("Adding {:?}", SubGrid::new(count, start, end));
    grids.push(SubGrid::new(count, start, end, depth + 1));
}

fn main() {
    let text = read_input::read_text("23/input.txt").unwrap();

    let mut nanobots = Vec::new();
    let mut largest_radius_with_index = (0, 0);
    for line in text.lines() {
        let mut sections = line.split(", ");
        let pos_text = sections.next().unwrap();
        let pos_text = pos_text.replace("pos=<", "").replace(">", "");

        let coords: Vec<i64> = pos_text.split(",").map(|n| n.parse().unwrap()).collect();

        let radius_text = sections.next().unwrap();
        let radius: i64 = radius_text.replace("r=", "").parse().unwrap();
        if radius > largest_radius_with_index.0 {
            largest_radius_with_index.0 = radius;
            largest_radius_with_index.1 = nanobots.len();
        }
        nanobots.push(Nanobot {
            pos: (coords[0], coords[1], coords[2]),
            radius,
        });
    }

    let largest_bot = nanobots.get(largest_radius_with_index.1).unwrap();
    let mut part_one_count = 0;

    let mut min_coord = 0;
    let mut max_coord = 0;

    for bot in nanobots.iter() {
        min_coord = cmp::min(min_coord, bot.pos.0);
        min_coord = cmp::min(min_coord, bot.pos.1);
        min_coord = cmp::min(min_coord, bot.pos.2);

        max_coord = cmp::max(max_coord, bot.pos.0);
        max_coord = cmp::max(max_coord, bot.pos.1);
        max_coord = cmp::max(max_coord, bot.pos.2);
        if distance(&largest_bot.pos, &bot.pos) <= largest_bot.radius {
            part_one_count += 1;
        }
    }

    println!("{}", part_one_count);

    let mut grids = {
        let min_x = min_coord;
        let min_y = min_coord;
        let min_z = min_coord;

        let mut max_x = max_coord;
        let mut max_y = max_coord;
        let mut max_z = max_coord;

        if (max_x - min_x) % 2 != 0 {
            max_x += 1;
        }
        if (max_y - min_y) % 2 != 0 {
            max_y += 1;
        }
        if (max_z - min_z) % 2 != 0 {
            max_z += 1;
        }

        let mut grids: BinaryHeap<SubGrid> = BinaryHeap::new();
        grids.push(SubGrid::new(nanobots.len() as i64, (min_x, min_y, min_z), (max_x, max_y, max_z), 0));

        grids
    };

    let mut count = 0;
    let mut closest_sum_for_count: HashMap<i64, i64> = HashMap::new();
    let mut single_sized_rects_scanned = HashSet::new();

    while let Some(sub_grid) = grids.pop() {
        let (max_x, max_y, max_z) = sub_grid.end;
        let (min_x, min_y, min_z) = sub_grid.start;
        let depth = sub_grid.depth;
        let mut size = max_x - min_x;

        // println!("Try {:?} with {}", sub_grid, size);

        // would be zero as the max and min
        if size == 0 {
            let manhattan_sum = (sub_grid.start.0).abs() + (sub_grid.start.1).abs() + (sub_grid.start.2).abs();
            if sub_grid.count > count {
                count = sub_grid.count;
                println!("{} for {:?} = {}", sub_grid.count, sub_grid.start, manhattan_sum);
            } else if sub_grid.count == count && manhattan_sum < *closest_sum_for_count.get(&count).unwrap_or(&std::i64::MAX) {
                closest_sum_for_count.insert(count, manhattan_sum);
                count = sub_grid.count;
                println!("{} for {:?} = {}", sub_grid.count, sub_grid.start, manhattan_sum);
            }
        } else {
            if size >= 2 {
                size /= 2;
            } else {
                // if size was already 1, that means we need to add size 0, single square
                size = 0;
            }

            let iter_size = cmp::max(1, size) as usize;
            for sub_x in (min_x..=max_x).step_by(iter_size) {
                for sub_y in (min_y..=max_y).step_by(iter_size) {
                    for sub_z in (min_z..=max_z).step_by(iter_size) {
                        if single_sized_rects_scanned.contains(&(sub_x, sub_y, sub_z, size)) {
                            continue
                        }
                        single_sized_rects_scanned.insert((sub_x, sub_y, sub_z, size));
                        check_rect(
                            (sub_x, sub_y, sub_z),
                            (sub_x + size, sub_y + size, sub_z + size),
                            &nanobots,
                            &mut grids,
                            depth
                        );
                    }
                }
            }
        }
    }
}
