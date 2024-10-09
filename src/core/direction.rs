#[derive(Debug, PartialEq, Clone)]
pub enum Direction {
    Left,
    Right,
    Stop,
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Stop
    }
}

