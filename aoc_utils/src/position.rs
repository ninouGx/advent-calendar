use crate::Direction;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position { x, y }
    }

    pub fn get_adjacent(&self, direction: &Direction) -> Position {
        let (dx, dy) = direction.delta();
        Position {
            x: self.x + dx,
            y: self.y + dy,
        }
    }
}
