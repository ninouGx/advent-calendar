use std::str::FromStr;
use crate::{ Direction, Position };

pub struct Grid {
    pub data: Vec<char>,
    pub rows: usize,
    pub cols: usize,
}

impl Grid {
    pub fn is_pos_valid(&self, pos: &Position) -> bool {
        pos.x >= 0 && pos.y >= 0 && (pos.x as usize) < self.cols && (pos.y as usize) < self.rows
    }

    pub fn get_index(&self, pos: &Position) -> Option<usize> {
        if !self.is_pos_valid(pos) {
            return None;
        }
        let index = (pos.y as usize) * self.cols + (pos.x as usize);
        Some(index)
    }

    pub fn get(&self, pos: &Position) -> Option<char> {
        let index = self.get_index(pos)?;
        Some(self.data[index])
    }

    pub fn get_position_index(&self, index: usize) -> Position {
        let x = (index % self.cols) as i32;
        let y = (index / self.cols) as i32;
        Position { x, y }
    }

    pub fn set(&mut self, pos: &Position, value: char) {
        if let Some(index) = self.get_index(pos) {
            self.data[index] = value;
        }
    }

    pub fn check_position(&self, pos: &Position, dir: &Direction) -> Option<char> {
        let new_pos = pos.get_adjacent(dir);
        self.get(&new_pos)
    }

    pub fn check_all_directions(&self, pos: &Position) -> Vec<char> {
        Direction::all()
            .filter_map(|dir| self.check_position(pos, &dir))
            .collect()
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
