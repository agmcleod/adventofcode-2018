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
    sub_x_size: i64,
    sub_y_size: i64,
    sub_z_size: i64,
    nanobots: &Vec<Nanobot>,
    highest_count: &mut Vec<(i64, Coord, Coord)>,
) {
    let start = (
        sub_x,
        sub_y,
        sub_z,
    );
    let end = (
        sub_x + sub_x_size,
        sub_y + sub_y_size,
        sub_z + sub_z_size,
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
    let text = read_input::read_text("23/edgecase3.txt").unwrap();

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

    let mut min_x = min_coord;
    let mut min_y = min_coord;
    let mut min_z = min_coord;

    let mut max_x = max_coord;
    let mut max_y = max_coord;
    let mut max_z = max_coord;

    println!("{}", part_one_count);

    if (max_x - min_x) % 2 != 0 {
        max_x += 1;
    }
    if (max_y - min_y) % 2 != 0 {
        max_y += 1;
    }
    if (max_z - min_z) % 2 != 0 {
        max_z += 1;
    }

    loop {
        let size_x = (max_x - min_x) / 2;
        let size_y = (max_y - min_y) / 2;
        let size_z = (max_z - min_z) / 2;

        let mut highest_count: Vec<(i64, Coord, Coord)> = Vec::new();

        for sub_x in (min_x..=max_x).step_by(size_x as usize) {
            for sub_y in (min_y..=max_y).step_by(size_y as usize) {
                for sub_z in (min_z..=max_z).step_by(size_z as usize) {
                    check_rect(
                        sub_x,
                        sub_y,
                        sub_z,
                        size_x,
                        size_y,
                        size_z,
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
            if obj.0 == first.0 && distance(&obj.1, &(0, 0, 0)) < distance(&coord.1, &(0, 0, 0)) {
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
            (size_x, size_y, size_z),
            first.0
        );

        min_x = (coord.1).0;
        min_y = (coord.1).1;
        min_z = (coord.1).2;

        max_x = (coord.2).0;
        max_y = (coord.2).1;
        max_z = (coord.2).2;

        if size_x == 1 && size_y == 1 && size_z == 1 {
            println!(
            "Final check {} {} {} {} {} {}",
                min_x,
                min_y,
                min_z,
                max_x,
                max_y,
                max_z,
            );
            let mut highest_count = Vec::new();
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    for z in min_z..=max_z {
                        check_rect(x, y, z, 0, 0, 0, &nanobots, &mut highest_count);
                    }
                }
            }

            highest_count.sort_by(|a, b| b.0.cmp(&a.0));

            let first = highest_count.get(0).unwrap();

            let mut coord = first;
            for obj in highest_count.iter().skip(1) {
                if obj.0 == first.0 && distance(&obj.1, &(0, 0, 0)) < distance(&coord.1, &(0, 0, 0)) {
                    coord = obj;
                }
            }

            min_x = (coord.1).0;
            min_y = (coord.1).1;
            min_z = (coord.1).2;

            break;
        }
    }

    println!(
        "Part Two: {} {} {}, {}",
        min_x,
        min_y,
        min_z,
        min_x.abs() + min_y.abs() + min_z.abs()
    );
}


#[test]
fn test_check_rect() {
    let bots = vec![
        Nanobot{ pos: (10, 12, 12), radius: 2 },
        Nanobot{ pos: (12, 14, 12), radius: 2 },
        Nanobot{ pos: (16, 12, 12), radius: 2 },
        Nanobot{ pos: (14, 14, 14), radius: 6 },
        Nanobot{ pos: (50, 50, 50), radius: 200 },
        Nanobot{ pos: (10, 10, 10), radius: 5 },
    ];

    let mut highest_count = Vec::new();
    check_rect(12, 12, 12, 1, 1, 1, &bots, &mut highest_count);

    assert_eq!(highest_count.get(0).unwrap().1, (12, 12, 12));
}