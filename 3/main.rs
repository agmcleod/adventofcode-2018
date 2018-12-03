extern crate read_input;

struct Vector2 {
    x: usize,
    y: usize,
}

struct Claim {
    id: usize,
    pos: Vector2,
    size: Vector2,
}

impl Claim {
    fn from_text(id: usize, text: &str) -> Self {
        let transform = text.split("@ ").nth(1).unwrap();
        let mut pieces = transform.split(": ");
        let mut pos = Vector2{x: 0, y: 0};
        let mut coords = pieces.next().unwrap().split(",");
        pos.x = coords.next().unwrap().parse().expect("Could not parse x");
        pos.y = coords.next().unwrap().parse().expect("Could not parse y");

        let mut size = Vector2{x: 0, y: 0};
        let mut coords = pieces.next().unwrap().split("x");
        size.x = coords.next().unwrap().parse().expect("Could not parse w");
        size.y = coords.next().unwrap().parse().expect("Could not parse h");

        Claim{
            id,
            pos,
            size,
        }
    }
}

fn main() {
    let input = read_input::read_text("3/input.txt").unwrap();

    let claims: Vec<Claim> = input.lines().enumerate().map(|(i, line)| {
        Claim::from_text(i + 1, line)
    }).collect();

    println!("{}", claims.len());
}