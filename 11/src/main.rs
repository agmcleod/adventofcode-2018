use std::cmp;
use std::collections::HashMap;

fn get_power(x: i32, y: i32) -> i32 {
    let rack_id = x + 10;
    let result = (rack_id * y + 9435) * rack_id;
    let power = if result < 100 {
        0
    } else {
        format!("{}", result)
            .rsplit("")
            .nth(3)
            .unwrap()
            .parse()
            .unwrap()
    };

    power - 5
}

fn create_sum_area_table(size: i32) -> HashMap<(i32, i32), i32> {
    let mut sum_area_table = HashMap::new();
    for x in 1..=size {
        for y in 1..=size {
            let sum = get_power(x, y)
                + get_value_to_add(&sum_area_table, x, y - 1)
                + get_value_to_add(&sum_area_table, x - 1, y)
                - get_value_to_add(&sum_area_table, x - 1, y - 1);
            sum_area_table.insert((x, y), sum);
        }
    }

    sum_area_table
}

fn get_value_to_add(sum_area_table: &HashMap<(i32, i32), i32>, x: i32, y: i32) -> i32 {
    if sum_area_table.contains_key(&(x, y)) {
        *sum_area_table.get(&(x, y)).unwrap()
    } else {
        0
    }
}

fn sum_for_grid(sum_area_table: &HashMap<(i32, i32), i32>, grid_size: i32, x: i32, y: i32) -> i32 {
    let g = grid_size - 1;
    get_value_to_add(sum_area_table, x + g, y + g) + get_value_to_add(sum_area_table, x - 1, y - 1)
        - get_value_to_add(sum_area_table, x + g, y - 1)
        - get_value_to_add(sum_area_table, x - 1, y + g)
}

fn sol(sum_area_table: &HashMap<(i32, i32), i32>, grid_size: i32) -> (i32, (i32, i32)) {
    let mut sum_with_coords = HashMap::new();

    let mut highest = 0;
    sum_with_coords.insert(0, (-1, -1));

    // case for handling 300x300
    if grid_size == 300 {
        let sum = sum_for_grid(sum_area_table, grid_size, 1, 1);
        highest = cmp::max(sum, highest);
        sum_with_coords.insert(sum, (1, 1));
    } else {
        for x in 1..=(300 - grid_size) {
            for y in 1..=(300 - grid_size) {
                let sum = sum_for_grid(sum_area_table, grid_size, x, y);
                highest = cmp::max(sum, highest);
                sum_with_coords.insert(sum, (x, y));
            }
        }
    }

    (highest, sum_with_coords.get(&highest).unwrap().clone())
}

fn main() {
    let sum_area_table = create_sum_area_table(300);

    let part_one = sol(&sum_area_table, 3);

    println!("{} {:?}", part_one.0, part_one.1);

    let mut highest_sum = part_one.0;
    let mut highest_sum_grid_size = 3;
    let mut highest_coord = (0, 0);
    for size in 1..=300 {
        let result = sol(&sum_area_table, size);
        if result.0 > highest_sum {
            highest_sum = result.0;
            highest_sum_grid_size = size;
            highest_coord = result.1;
        }
    }

    println!(
        "{} {} {:?}",
        highest_sum, highest_sum_grid_size, highest_coord
    );
}
