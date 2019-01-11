use read_input;

struct Nanobot {
    pos: (i64, i64, i64),
    radius: i64,
}

fn distance(location: &(i64, i64, i64), target: &(i64, i64, i64)) -> i64 {
    let mut x_diff = (location.0 - target.0).abs();
    let mut y_diff = (location.1 - target.1).abs();
    let mut z_diff = (location.2 - target.2).abs();

    x_diff + y_diff + z_diff
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
    for bot in nanobots.iter() {
        if distance(&largest_bot.pos, &bot.pos) <= largest_bot.radius {
            part_one_count += 1;
        }
    }

    println!("{}", part_one_count);

    // for (i, bot) in nanobots.iter().enumerate() {
    //     for bot2 in nanobots.iter().skip(i) {

    //     }
    // }
}
