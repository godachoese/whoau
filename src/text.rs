pub enum Direction {
    Left,
    Right,
    Center,
}

pub fn align(text: &str, direction: Direction, length: usize) -> String {
    match direction {
        Direction::Left => format!("{:<length$}", text),
        Direction::Right => format!("{:>length$}", text),
        Direction::Center => format!("{:^length$}", text),
    }
}
