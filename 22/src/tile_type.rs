#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TileType {
    Rocky,
    Wet,
    Narrow
}

impl TileType {
    pub fn from_num(num: usize) -> TileType {
        match num {
            0 => TileType::Rocky,
            1 => TileType::Wet,
            2 => TileType::Narrow,
            _ => panic!("Invalid number"),
        }
    }
}
