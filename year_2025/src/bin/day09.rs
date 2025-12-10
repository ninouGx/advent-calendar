use std::{ collections::HashSet, time::Duration };
use aoc_utils::{ Position, create_pairs };
use progress_timer::time_function;
use rayon::prelude::*;

struct Rectangle {
    corners: (Position, Position),
}

impl Rectangle {
    fn get_area(&self) -> usize {
        let width = ((self.corners.0.x - self.corners.1.x).abs() as usize) + 1;
        let height = ((self.corners.0.y - self.corners.1.y).abs() as usize) + 1;
        width * height
    }

    fn get_pos_inside(&self) -> HashSet<Position> {
        let mut positions = HashSet::new();
        let x_start = self.corners.0.x.min(self.corners.1.x);
        let x_end = self.corners.0.x.max(self.corners.1.x);
        let y_start = self.corners.0.y.min(self.corners.1.y);
        let y_end = self.corners.0.y.max(self.corners.1.y);
        for x in x_start..=x_end {
            for y in y_start..=y_end {
                positions.insert(Position { x, y });
            }
        }
        positions
    }
}

fn get_polygon_inside_positions(polygon: &Vec<Position>) -> HashSet<Position> {
    let mut inside_positions = HashSet::new();
    let min_x = polygon
        .iter()
        .map(|pos| pos.x)
        .min()
        .unwrap();
    let max_x = polygon
        .iter()
        .map(|pos| pos.x)
        .max()
        .unwrap();
    let min_y = polygon
        .iter()
        .map(|pos| pos.y)
        .min()
        .unwrap();
    let max_y = polygon
        .iter()
        .map(|pos| pos.y)
        .max()
        .unwrap();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let pos = Position { x, y };
            if is_inside_polygon(&pos, polygon) {
                inside_positions.insert(pos);
            }
        }
    }
    inside_positions
}

/// Determines if a point is inside a polygon using the Ray Casting algorithm.
///
/// How it works:
/// 1. Cast a horizontal ray from the point to the right (toward +infinity)
/// 2. Count how many polygon edges the ray crosses
/// 3. Odd crossings = inside, Even crossings = outside
fn is_inside_polygon(pos: &Position, polygon: &Vec<Position>) -> bool {
    let mut crossing_count = 0;

    for i in 0..polygon.len() - 1 {
        let edge_start = &polygon[i];
        let edge_end = &polygon[i + 1];

        // Check if this edge crosses our ray's y-level
        let min_y = edge_start.y.min(edge_end.y);
        let max_y = edge_start.y.max(edge_end.y);
        let edge_spans_our_y = pos.y > min_y && pos.y <= max_y;

        if !edge_spans_our_y {
            continue;
        }

        // Calculate where this edge crosses our y-level (the x coordinate)
        // Linear interpolation: x = x1 + (x2-x1) * (y-y1) / (y2-y1)
        let delta_x = edge_end.x - edge_start.x;
        let delta_y = edge_end.y - edge_start.y;

        if delta_y == 0 {
            // Horizontal edge, doesn't count as a crossing
            continue;
        }

        let y_offset = pos.y - edge_start.y;
        let x_intersection = edge_start.x + (delta_x * y_offset) / delta_y;

        // Check if this intersection is to the right of pos
        if x_intersection > pos.x {
            crossing_count += 1;
        }
    }

    // Odd number of crossings = inside the polygon
    crossing_count % 2 == 1
}

fn part1(input: &str) -> usize {
    let positions: Vec<Position> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    create_pairs(&positions)
        .into_iter()
        .map(|(pos1, pos2)| {
            let rect = Rectangle {
                corners: (*pos1, *pos2),
            };
            rect.get_area()
        })
        .max()
        .unwrap_or(0)
}

fn part2(input: &str) -> usize {
    let mut red_positions: Vec<Position> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    red_positions.push(red_positions[0].clone());

    let mut green_positions: HashSet<Position> = red_positions.iter().cloned().collect();
    red_positions.windows(2).for_each(|pair| {
        let pos1 = &pair[0];
        let pos2 = &pair[1];
        let is_same_row = pos1.y == pos2.y;
        if is_same_row {
            (pos1.x.min(pos2.x)..=pos1.x.max(pos2.x)).for_each(|x| {
                let pos = Position { x, y: pos1.y };
                green_positions.insert(pos);
            });
        } else {
            (pos1.y.min(pos2.y)..=pos1.y.max(pos2.y)).for_each(|y| {
                let pos = Position { x: pos1.x, y };
                green_positions.insert(pos);
            });
        }
    });

    let mut candidate_positions: Vec<Position> = red_positions.iter().cloned().collect();
    candidate_positions.sort_unstable_by_key(|p| (p.x, p.y));
    candidate_positions.dedup();

    create_pairs(&candidate_positions)
        .par_iter()
        .filter_map(|(pos1, pos2)| {
            let x1 = pos1.x;
            let y1 = pos1.y;
            let x2 = pos2.x;
            let y2 = pos2.y;

            // Skip if not a valid rectangle
            if x1 == x2 || y1 == y2 {
                return None;
            }

            let x_start = x1.min(x2);
            let x_end = x1.max(x2);
            let y_start = y1.min(y2);
            let y_end = y1.max(y2);

            // Early filtering: check the 4 corners first
            // If any corner is outside, the whole rectangle is invalid
            let corner1 = Position { x: x_start, y: y_start };
            let corner2 = Position { x: x_end, y: y_start };
            let corner3 = Position { x: x_start, y: y_end };
            let corner4 = Position { x: x_end, y: y_end };

            for corner in [&corner1, &corner2, &corner3, &corner4] {
                if !green_positions.contains(corner) && !is_inside_polygon(corner, &red_positions) {
                    return None;
                }
            }

            // Now check ALL positions inside the rectangle
            let all_inside = (x_start..=x_end).all(|x| {
                (y_start..=y_end).all(|y| {
                    let pos = Position { x, y };
                    green_positions.contains(&pos) || is_inside_polygon(&pos, &red_positions)
                })
            });

            if all_inside {
                let width = (x_end - x_start + 1) as usize;
                let height = (y_end - y_start + 1) as usize;
                Some(width * height)
            } else {
                None
            }
        })
        .max()
        .unwrap_or(0)
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
        let input = r#"
2,5
11,1
"#.trim();
        let result = part1(input);
        assert_eq!(result, 50);
    }

    #[test]
    fn test_part1_example2() {
        let input = r#"
another
test
case
"#.trim();
        let result = part1(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_example1() {
        let input = r#"
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"#.trim();
        let result = part2(input);
        assert_eq!(result, 24);
    }

    #[test]
    fn test_part2_example2() {
        let input = r#"
another
part2
case
"#.trim();
        let result = part2(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part2_edge_case() {
        let input = r#"
edge
case
test
"#.trim();
        let result = part2(input);
        assert_eq!(result, 0);
    }
}
