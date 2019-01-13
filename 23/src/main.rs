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
    sub_x: i64,
    sub_y: i64,
    sub_z: i64,
    min_x: i64,
    min_y: i64,
    min_z: i64,
    sub_x_size: i64,
    sub_y_size: i64,
    sub_z_size: i64,
    nanobots: &Vec<Nanobot>,
    highest_count: &mut Vec<(i64, Coord, Coord)>,
) {
    let start = (
        (sub_x * sub_x_size) + min_x,
        (sub_y * sub_y_size) + min_y,
        (sub_z * sub_z_size) + min_z,
    );
    let end = (
        ((sub_x + 1) * sub_x_size) + min_x,
        ((sub_y + 1) * sub_y_size) + min_y,
        ((sub_z + 1) * sub_z_size) + min_z,
    );

    let mut count = 0;

    for bot in nanobots {
        let x = clamp_to_closest(bot.pos.0, start.0, end.0);
        let y = clamp_to_closest(bot.pos.1, start.1, end.1);
        let z = clamp_to_closest(bot.pos.2, start.2, end.2);

        if distance(&bot.pos, &(x, y, z)) <= bot.radius {
            count += 1;
        }
    }

    highest_count.push((count, start, end));
}

fn main() {
    let text = read_input::read_text("23/example.txt").unwrap();

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

    if (max_x - min_x) % 2 != 0 {
        max_x += 1;
    }
    if (max_y - min_y) % 2 != 0 {
        max_y += 1;
    }
    if (max_z - min_z) % 2 != 0 {
        max_z += 1;
    }

    println!("{}", part_one_count);
    println!(
        "Start of Part 2: {} {} {} {} {} {}",
        min_x, min_y, min_z, max_x, max_y, max_z
    );

    loop {
        let sub_x_size = (max_x - min_x).abs() / 2;
        let sub_y_size = (max_y - min_y).abs() / 2;
        let sub_z_size = (max_z - min_z).abs() / 2;

        let mut highest_count: Vec<(i64, Coord, Coord)> = Vec::new();

        for sub_x in 0..2 {
            for sub_y in 0..2 {
                for sub_z in 0..2 {
                    check_rect(
                        sub_x,
                        sub_y,
                        sub_z,
                        min_x,
                        min_y,
                        min_z,
                        sub_x_size,
                        sub_y_size,
                        sub_z_size,
                        &nanobots,
                        &mut highest_count,
                    );
                }
            }
        }

        highest_count.sort_by(|a, b| b.0.cmp(&a.0));

        let first = highest_count.get(0).unwrap();

        let mut coord = first;
        for obj in highest_count.iter().skip(1) {
            if obj.0 == first.0 && distance(&obj.1, &(0, 0, 0)) < distance(&first.1, &(0, 0, 0)) {
                coord = obj;
            }
        }

        println!(
            "{} {} {} {} {} {} size {:?} count {}",
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
            (sub_x_size, sub_y_size, sub_z_size),
            first.0
        );

        min_x = (coord.1).0;
        min_y = (coord.1).1;
        min_z = (coord.1).2;

        max_x = (coord.2).0;
        max_y = (coord.2).1;
        max_z = (coord.2).2;

        if max_x - min_x <= 1 && max_y - min_y <= 1 && max_z - min_z <= 1 {
            break;
        }
    }

    println!(
        "Part Two: {} {} {} {} {} {}, {}",
        min_x,
        min_y,
        min_z,
        max_x,
        max_y,
        max_z,
        min_x.abs() + min_y.abs() + min_z.abs()
    );
}
