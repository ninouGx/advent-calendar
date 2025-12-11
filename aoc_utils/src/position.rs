use std::str::FromStr;

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

impl FromStr for Position {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid position string: {}", s));
        }
        let x = parts[0]
            .trim()
            .parse::<i32>()
            .map_err(|e| e.to_string())?;
        let y = parts[1]
            .trim()
            .parse::<i32>()
            .map_err(|e| e.to_string())?;
        Ok(Position { x, y })
    }
}
