use read_input;

use std::cmp;

type Coord = (i64, i64, i64);

struct Nanobot {
    pos: Coord,
    radius: i64,
}

fn distance(location: &Coord, target: &Coord) -> i64 {
    let x_diff = (location.0 - target.0).abs();
    let y_diff = (location.1 - target.1).abs();
    let z_diff = (location.2 - target.2).abs();

    x_diff + y_diff + z_diff
}

fn rectangles_intersect(rect1_start: &Coord, rect1_end: &Coord, rect2_start: &Coord, rect2_end: &Coord) -> bool {
    rect1_start.0 <= rect2_start.0 &&
    rect1_start.1 <= rect2_start.1 &&
    rect1_start.2 <= rect2_start.2 &&
    rect1_end.0 >= rect1_end.0 &&
    rect1_end.1 >= rect1_end.1 &&
    rect1_end.2 >= rect1_end.2
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
        nanobots.push(Nanobot{ pos: (coords[0], coords[1], coords[2]) , radius });
    }

    let largest_bot = nanobots.get(largest_radius_with_index.1).unwrap();
    let mut part_one_count = 0;

    let mut min_x = 0;
    let mut min_y = 0;
    let mut min_z = 0;

    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_z = 0;

    for bot in nanobots.iter() {
        min_x = cmp::min(min_x, bot.pos.0);
        min_y = cmp::min(min_y, bot.pos.1);
        min_z = cmp::min(min_z, bot.pos.2);

        max_x = cmp::max(max_x, bot.pos.0);
        max_y = cmp::max(max_y, bot.pos.1);
        max_z = cmp::max(max_z, bot.pos.2);
        if distance(&largest_bot.pos, &bot.pos) <= largest_bot.radius {
            part_one_count += 1;
        }
    }

    println!("{}", part_one_count);
    println!("{} {} {} {} {} {}", min_x, min_y, min_z, max_x, max_y, max_z);

    loop {
        let sub_x_size = (max_x - min_x) / 8;
        let sub_y_size = (max_y - min_y) / 8;
        let sub_z_size = (max_z - min_z) / 8;

        let mut highest_count: Vec<(i64, Coord, Coord)> = Vec::new();

        for sub_x in 0..8 {
            for sub_y in 0..8 {
                for sub_z in 0..8 {
                    let start = ((sub_x * sub_x_size) + min_x, (sub_y * sub_y_size) + min_y, (sub_z * sub_z_size) + min_z);
                    let end = (((sub_x + 1) * sub_x_size) + min_x, ((sub_y + 1) * sub_y_size) + min_y, ((sub_z + 1) * sub_z_size) + min_z);

                    let mut count = 0;

                    for bot in &nanobots {
                        let start_bot = (bot.pos.0 - bot.radius, bot.pos.1 - bot.radius, bot.pos.2 - bot.radius);
                        let end_bot = (bot.pos.0 + bot.radius, bot.pos.1 + bot.radius, bot.pos.2 + bot.radius);

                        if rectangles_intersect(&start, &end, &start_bot, &end_bot) {
                            count += 1;
                        }
                    }

                    highest_count.push((count, start, end));
                }
            }
        }

        highest_count.sort_by(|a, b| {
            b.0.cmp(&a.0)
        });

        let first = highest_count.get(0).unwrap();

        println!("Count collided {}", first.0);
        
        min_x = (first.1).0;
        min_y = (first.1).1;
        min_z = (first.1).2;

        max_x = (first.2).0;
        max_y = (first.2).1;
        max_z = (first.2).2;

        if max_x - min_x <= 1 && max_y - min_y <= 1 && max_z - min_z <= 1 {
            break
        }
    }

    println!("{} {} {} {} {} {}", min_x, min_y, min_z, max_x, max_y, max_z);
}
