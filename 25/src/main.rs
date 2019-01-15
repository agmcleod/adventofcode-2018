use read_input;

type Coord = (i32, i32, i32, i32);

fn main() {
    let text = read_input::read_text("25/input.txt").unwrap();

    let coords: Vec<Coord> = text.lines().map(|line| {
        let nums: Vec<i32> = line.split(",").map(|n| n.parse().unwrap()).collect();
        (
            nums[0],
            nums[1],
            nums[2],
            nums[3],
        )
    }).collect();
}
