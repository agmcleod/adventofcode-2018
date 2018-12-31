extern crate read_input;

use std::cmp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;

fn get_range_numbers(second_coord: &str, coord_name: &str) -> Vec<i32> {
    second_coord
        .replace(coord_name, "")
        .split("..")
        .map(|v| v.parse().unwrap())
        .collect()
}

fn add_element_for_key_to_map(map: &mut HashMap<i32, Vec<i32>>, key: i32, element: i32) {
    if map.contains_key(&key) {
        let list = map.get_mut(&key).unwrap();
        list.push(element);
        list.sort();
    } else {
        map.insert(key, vec![-1, element]);
    }
}

fn write_map(
    file_path: &str,
    deepest_clay: i32,
    min_x: i32,
    max_x: i32,
    full_clay_map: &HashSet<(i32, i32)>,
    watered_area: &HashSet<(i32, i32)>,
    falling_water: &HashSet<(i32, i32)>,
) {
    let mut f = File::create(file_path).unwrap();
    let mut lines = Vec::new();
    for y in 0..=deepest_clay {
        let mut line = Vec::new();
        if y == 0 {
            for x in (min_x - 10)..=(max_x + 10) {
                if x < 500 {
                    line.push(".");
                } else {
                    line.push("500");
                    break;
                }
            }
        } else {
            for x in (min_x - 10)..=(max_x + 10) {
                if full_clay_map.contains(&(x, y)) {
                    line.push("#");
                } else if watered_area.contains(&(x, y)) {
                    line.push("~");
                } else if falling_water.contains(&(x, y)) {
                    line.push("|");
                } else {
                    line.push(".");
                }
            }
        }
        lines.push(line.join(""));
    }

    f.write_all(lines.join("\n").as_bytes()).unwrap();
}

fn main() {
    let text = read_input::read_text("17/input.txt").unwrap();

    let mut clay_by_x = HashMap::new();
    let mut clay_by_y = HashMap::new();
    let mut full_clay_map = HashSet::new();
    let mut deepest_clay = 0;
    let mut shallowest_clay = 10_000;
    let mut min_x = 501;
    let mut max_x = 499;

    for line in text.lines() {
        let mut coord_ranges = line.split(", ");
        let first_coord = coord_ranges.next().unwrap();
        let second_coord = coord_ranges.next().unwrap();
        let first_char = first_coord.chars().next().unwrap();
        if first_char == 'x' {
            let x: i32 = first_coord.replace("x=", "").parse().unwrap();
            let y_range = get_range_numbers(second_coord, "y=");

            for y in y_range[0]..=y_range[1] {
                add_element_for_key_to_map(&mut clay_by_x, x, y);
                add_element_for_key_to_map(&mut clay_by_y, y, x);
                deepest_clay = cmp::max(deepest_clay, y);
                shallowest_clay = cmp::min(shallowest_clay, y);
                full_clay_map.insert((x, y));
                min_x = cmp::min(min_x, x);
                max_x = cmp::max(max_x, x);
            }
        } else if first_char == 'y' {
            let y: i32 = first_coord.replace("y=", "").parse().unwrap();
            let x_range = get_range_numbers(second_coord, "x=");

            for x in x_range[0]..=x_range[1] {
                add_element_for_key_to_map(&mut clay_by_x, x, y);
                add_element_for_key_to_map(&mut clay_by_y, y, x);
                deepest_clay = cmp::max(deepest_clay, y);
                shallowest_clay = cmp::min(shallowest_clay, y);
                full_clay_map.insert((x, y));
                min_x = cmp::min(min_x, x);
                max_x = cmp::max(max_x, x);
            }
        } else {
            panic!("Could not parse {}", first_coord);
        }
    }

    let mut current_vector = vec![(500, 1)];
    let mut watered_area = HashSet::new();
    let mut falling_water = HashSet::new();
    let mut failing_vectors = HashSet::new();

    write_map(
        "17/claymap.txt",
        deepest_clay,
        min_x,
        max_x,
        &full_clay_map,
        &watered_area,
        &falling_water,
    );

    loop {
        if let Some(mut vec) = current_vector.pop() {
            failing_vectors.insert(vec.clone());
            let mut found_y_collision = false;
            if clay_by_x.contains_key(&vec.0) {
                // println!("Try {:?}", vec.1);
                for two_ys in clay_by_x.get(&vec.0).unwrap().windows(2) {
                    if vec.1 > two_ys[0] && vec.1 < two_ys[1] {
                        found_y_collision = true;
                        for y in vec.1..two_ys[1] {
                            if y >= shallowest_clay {
                                falling_water.insert((vec.0, y));
                            }
                        }
                        // subtract 1 as we dont want to include the clay itself
                        vec.1 = two_ys[1] - 1;
                    }
                }

                if found_y_collision {
                    let mut last_min = vec.0 - 1;
                    let mut last_max = vec.0 + 1;
                    'scale_walls: loop {
                        let mut min_x = vec.0 - 1;
                        let mut clay_walls_found = 0;
                        loop {
                            // next space is above no space
                            // this assumes that a wall is always above another clay tile
                            // also checks if tile below is not water
                            if min_x <= last_min
                                && !full_clay_map.contains(&(min_x, vec.1 + 1))
                                && !watered_area.contains(&(min_x, vec.1 + 1))
                            {
                                // since there's no ground start this as a new stream
                                if !failing_vectors.contains(&(min_x, vec.1)) {
                                    current_vector.push((min_x, vec.1));
                                }
                                last_min = min_x;
                                break;
                            // min x is now a clay wall
                            } else if full_clay_map.contains(&(min_x, vec.1)) {
                                // we dont add the + 1 here, as we want to include the claywall space
                                last_min = min_x;
                                // we add one back, because the clay is in that spot
                                min_x += 1;
                                clay_walls_found += 1;
                                break;
                            }
                            min_x -= 1;
                        }

                        let mut max_x = vec.0 + 1;
                        loop {
                            if max_x >= last_max
                                && !full_clay_map.contains(&(max_x, vec.1 + 1))
                                && !watered_area.contains(&(max_x, vec.1 + 1))
                            {
                                // since there's no ground start this as a new stream
                                if !failing_vectors.contains(&(max_x, vec.1)) {
                                    current_vector.push((max_x, vec.1));
                                }
                                last_max = max_x;
                                break;
                            } else if full_clay_map.contains(&(max_x, vec.1)) {
                                // we dont subtract the 1 here, as we want to include the claywall space
                                last_max = max_x;
                                // subtract one in this case, dont count the clay wall!
                                max_x -= 1;
                                clay_walls_found += 1;
                                break;
                            }
                            max_x += 1;
                        }

                        for x in min_x..=max_x {
                            if clay_walls_found == 2 {
                                watered_area.insert((x, vec.1));
                            } else {
                                falling_water.insert((x, vec.1));
                            }
                        }

                        if clay_walls_found == 2 {
                            vec.1 -= 1;
                        } else {
                            break 'scale_walls;
                        }
                    }
                }
            }
            if !found_y_collision {
                for y in vec.1..=deepest_clay {
                    falling_water.insert((vec.0, y));
                }
            }
        } else {
            break;
        }
    }

    // we check overlap to not double count
    let watered_area_sum = watered_area.iter().fold(0, |sum, coord| {
        if !falling_water.contains(coord) {
            sum + 1
        } else {
            sum
        }
    });
    let sum = falling_water.len() + watered_area_sum;
    println!("{} - {}", sum, watered_area.len());
    write_map(
        "17/claymap-full.txt",
        deepest_clay,
        min_x,
        max_x,
        &full_clay_map,
        &watered_area,
        &falling_water,
    );
}
