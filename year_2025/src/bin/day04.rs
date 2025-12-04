use std::{ str::FromStr, time::Duration };
use progress_timer::time_function;

enum Direction {
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
    fn delta(&self) -> (i32, i32) {
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
}

struct Position {
    x: i32,
    y: i32,
}

struct Grid {
    data: Vec<char>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn is_pos_valid(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.y >= 0 && (pos.x as usize) < self.cols && (pos.y as usize) < self.rows
    }

    fn get_index(&self, pos: &Position) -> Option<usize> {
        if !self.is_pos_valid(pos) {
            return None;
        }
        let index = (pos.y as usize) * self.cols + (pos.x as usize);
        Some(index)
    }

    fn get(&self, pos: &Position) -> Option<char> {
        let index = match self.get_index(pos) {
            Some(idx) => idx,
            None => {
                return None;
            }
        };
        Some(self.data[index])
    }

    fn get_position_index(&self, index: usize) -> Position {
        let x = (index % self.cols) as i32;
        let y = (index / self.cols) as i32;
        Position { x, y }
    }

    fn set(&mut self, pos: &Position, value: char) {
        if let Some(index) = self.get_index(pos) {
            self.data[index] = value;
        }
    }

    fn check_position(&self, pos: &Position, dir: &Direction) -> Option<char> {
        let (dx, dy) = dir.delta();
        let new_pos = Position {
            x: pos.x + dx,
            y: pos.y + dy,
        };
        self.get(&new_pos)
    }

    fn check_all_directions(&self, pos: &Position) -> Vec<char> {
        let directions = vec![
            Direction::North,
            Direction::NorthEast,
            Direction::East,
            Direction::SouthEast,
            Direction::South,
            Direction::SouthWest,
            Direction::West,
            Direction::NorthWest
        ];
        let mut results = Vec::new();
        for dir in directions {
            if let Some(c) = self.check_position(pos, &dir) {
                results.push(c);
            }
        }
        results
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cols = s
            .lines()
            .next()
            .map(|l| l.len())
            .unwrap_or(0);
        let rows = s.lines().count();

        if rows == 0 {
            return Err("Empty grid".to_string());
        }

        let data: Vec<char> = s
            .lines()
            .flat_map(|line| line.chars())
            .collect();

        Ok(Grid { data, rows, cols })
    }
}

const ROLL_CHAR: char = '@';

fn part1(input: &str) -> usize {
    let grid = Grid::from_str(input).unwrap();
    grid.data
        .iter()
        .enumerate()
        .filter(|&(ref i, &c)| {
            if c != ROLL_CHAR {
                return false;
            }
            let pos = grid.get_position_index(*i);
            let neighbors = grid.check_all_directions(&pos);
            neighbors
                .iter()
                .filter(|&c| c == &ROLL_CHAR)
                .count() < 4
        })
        .count()
}

fn part2(input: &str) -> usize {
    let mut grid = Grid::from_str(input).unwrap();
    let mut sum = 0;
    loop {
        let indices_to_remove: Vec<usize> = grid.data
            .iter()
            .enumerate()
            .filter(|&(ref i, &c)| {
                if c != ROLL_CHAR {
                    return false;
                }
                let pos = grid.get_position_index(*i);
                let neighbors = grid.check_all_directions(&pos);
                neighbors
                    .iter()
                    .filter(|&c| c == &ROLL_CHAR)
                    .count() < 4
            })
            .map(|(i, _)| i)
            .collect();

        for i in &indices_to_remove {
            let pos = grid.get_position_index(*i);
            grid.set(&pos, '.');
        }

        let removed_count = indices_to_remove.len();
        if removed_count == 0 {
            break;
        }
        sum += removed_count;
    }
    sum
}

fn main() {
    let is_test = false;
    let input = aoc_utils::get_input_for_day(is_test);
    let result_part_1 = time_function(
        "Part 1",
        Duration::from_secs(5),
        Duration::from_millis(100),
        || part1(&input)
    );
    println!("Part 1: {}", result_part_1);

    let result_part_2 = time_function(
        "Part 2",
        Duration::from_secs(5),
        Duration::from_millis(100),
        || part2(&input)
    );
    println!("Part 2: {}", result_part_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example1() {
        let input = "\
@.@
.@.
@.@";
        let result = part1(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part1_example2() {
        let input = "\
@@@
@@@
@@@";
        let result = part1(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part2_example1() {
        let input = "\
part2
test
input";
        let result = part2(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_example2() {
        let input = "\
another
part2
case";
        let result = part2(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_edge_case() {
        let input = "\
edge
case
test";
        let result = part2(input);
        assert_eq!(result, 0);
    }
}
