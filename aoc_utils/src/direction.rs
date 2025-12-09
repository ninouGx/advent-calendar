#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    pub fn delta(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::NorthEast => (1, -1),
            Direction::East => (1, 0),
            Direction::SouthEast => (1, 1),
            Direction::South => (0, 1),
            Direction::SouthWest => (-1, 1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (-1, -1),
        }
    }

    /// Returns an iterator over all 8 directions
    pub fn all() -> impl Iterator<Item = Direction> {
        [
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest,
        ].into_iter()
    }

    /// Returns an iterator over the 4 cardinal directions (N, E, S, W)
    pub fn cardinals() -> impl Iterator<Item = Direction> {
        [Direction::North, Direction::East, Direction::South, Direction::West].into_iter()
    }

    /// Returns an iterator over the 4 diagonal directions (NE, SE, SW, NW)
    pub fn diagonals() -> impl Iterator<Item = Direction> {
        [
            Direction::NorthEast,
            Direction::SouthEast,
            Direction::SouthWest,
            Direction::NorthWest,
        ].into_iter()
    }
}
