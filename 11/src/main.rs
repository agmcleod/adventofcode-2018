use std::collections::HashMap;
use std::cmp;

fn get_power(x: i32, y: i32) -> i32 {
    let rack_id = x + 10;
    let result = (rack_id * y + 9435) * rack_id;
    if result < 100 {
        0
    } else {
        format!("{}", result).rsplit("").nth(3).unwrap().parse().unwrap()
    }
}

fn sol(fuel_cell_totals: &mut HashMap<(i32, i32), i32>, grid_size: i32) -> (i32, (i32, i32)) {
    let mut sum_with_coords = HashMap::new();

    let mut highest = 0;

    for x in 0..=(300 - grid_size + 1) {
        for y in 0..=(298 - grid_size + 1) {

            let mut sum = 0;
            for x in x..=(x + grid_size - 1) {
                for y in y..=(y + grid_size - 1) {
                    let power = if fuel_cell_totals.contains_key(&(x, y)) {
                        *fuel_cell_totals.get(&(x, y)).unwrap()
                    } else {
                        let power = get_power(x, y);
                        fuel_cell_totals.insert((x, y), power);
                        power
                    };

                    sum += power;
                }
            }

            highest = cmp::max(sum, highest);
            sum_with_coords.insert(sum, (x, y));
        }
    }

    (highest, sum_with_coords.get(&highest).unwrap().clone())
}

fn main() {
    let mut fuel_cell_totals = HashMap::new();
    let part_one = sol(&mut fuel_cell_totals, 3);

    println!("{} {:?}", part_one.0, part_one.1);

    let mut highest_sum = part_one.0;
    let mut highest_sum_grid_size = 3;
    let mut highest_coord = (0, 0);
    for size in 1..=300 {
        if size == 3 {
            continue
        }

        let result = sol(&mut fuel_cell_totals, size);
        if result.0 > highest_sum {
            highest_sum = result.0;
            highest_sum_grid_size = size;
            highest_coord = result.1;
        }
    }

    println!("{} {} {:?}", highest_sum, highest_sum_grid_size, highest_coord);
}
