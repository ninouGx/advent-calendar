use std::time::Duration;
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

fn part2_original(input: &str) -> usize {
    use std::sync::atomic::{ AtomicUsize, Ordering };
    use std::sync::Arc;

    let mut red_positions: Vec<Position> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    red_positions.push(red_positions[0].clone());

    // Build the full perimeter (for validation) - optimized with capacity hint
    let mut green_positions: std::collections::HashSet<Position> = std::collections::HashSet::with_capacity(
        red_positions.len() * 100
    );
    green_positions.extend(red_positions.iter().cloned());

    red_positions.windows(2).for_each(|pair| {
        let pos1 = &pair[0];
        let pos2 = &pair[1];
        let is_same_row = pos1.y == pos2.y;
        if is_same_row {
            (pos1.x.min(pos2.x)..=pos1.x.max(pos2.x)).for_each(|x| {
                green_positions.insert(Position { x, y: pos1.y });
            });
        } else {
            (pos1.y.min(pos2.y)..=pos1.y.max(pos2.y)).for_each(|y| {
                green_positions.insert(Position { x: pos1.x, y });
            });
        }
    });

    // Use only polygon VERTICES as rectangle corners
    let mut candidate_positions: Vec<Position> = red_positions.iter().cloned().collect();
    candidate_positions.sort_unstable_by_key(|p| (p.x, p.y));
    candidate_positions.dedup();

    let total_pairs = (candidate_positions.len() * (candidate_positions.len() - 1)) / 2;
    println!("ORIGINAL APPROACH: Checking {} vertex pairs...", total_pairs);
    println!("This uses the slow point-by-point validation method.");
    println!("Progress updates every 1%:\n");

    // Atomic counter for accurate progress tracking across threads
    let progress_counter = Arc::new(AtomicUsize::new(0));
    let progress_interval = total_pairs / 100; // Report every 1%

    let max_area = create_pairs(&candidate_positions)
        .par_iter()
        .filter_map(|(pos1, pos2)| {
            // Atomic progress tracking
            let current = progress_counter.fetch_add(1, Ordering::Relaxed);
            if current % progress_interval == 0 && current > 0 {
                let progress = (current * 100) / total_pairs;
                eprintln!("Progress: {}% ({}/{} pairs)", progress, current, total_pairs);
            }

            // Skip invalid rectangles early
            if pos1.x == pos2.x || pos1.y == pos2.y {
                return None;
            }

            let x_start = pos1.x.min(pos2.x);
            let x_end = pos1.x.max(pos2.x);
            let y_start = pos1.y.min(pos2.y);
            let y_end = pos1.y.max(pos2.y);

            // Early filtering: check the 4 corners first (fast rejection)
            let corners = [
                Position { x: x_start, y: y_start },
                Position { x: x_end, y: y_start },
                Position { x: x_start, y: y_end },
                Position { x: x_end, y: y_end },
            ];

            for corner in &corners {
                if !green_positions.contains(corner) && !is_inside_polygon(corner, &red_positions) {
                    return None; // Early exit - invalid corner
                }
            }

            // ORIGINAL SLOW APPROACH: Check ALL interior points
            // This is what makes it slow - potentially billions of point checks
            let all_inside = (x_start..=x_end).all(|x| {
                (y_start..=y_end).all(|y| {
                    let pos = Position { x, y };
                    // Try fast path first (boundary check), then slow path (ray-casting)
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
        .unwrap_or(0);

    println!("Progress: 100% ({}/{} pairs) - COMPLETE!", total_pairs, total_pairs);
    max_area
}

fn part2(input: &str) -> usize {
    let mut polygon: Vec<Position> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    polygon.push(polygon[0].clone());

    // Step 1: Extract unique x and y coordinates (coordinate compression)
    let mut unique_x: Vec<i32> = polygon
        .iter()
        .map(|p| p.x)
        .collect();
    unique_x.sort_unstable();
    unique_x.dedup();

    let mut unique_y: Vec<i32> = polygon
        .iter()
        .map(|p| p.y)
        .collect();
    unique_y.sort_unstable();
    unique_y.dedup();

    // Step 2: Build compressed grid marking inside/boundary cells
    let compressed_width = unique_x.len();
    let compressed_height = unique_y.len();
    let mut grid = vec![vec![false; compressed_width]; compressed_height];

    // Mark all points in the polygon (boundary + interior)
    for (comp_y, &actual_y) in unique_y.iter().enumerate() {
        for (comp_x, &actual_x) in unique_x.iter().enumerate() {
            let pos = Position { x: actual_x, y: actual_y };

            // Check if this point is on the boundary or inside
            let on_boundary = polygon
                .windows(2)
                .any(|edge| { is_point_on_segment(&pos, edge[0], edge[1]) });

            let inside = !on_boundary && is_inside_polygon(&pos, &polygon);

            grid[comp_y][comp_x] = on_boundary || inside;
        }
    }

    // Step 3: Check all pairs of vertices for maximum rectangle
    let vertices: Vec<Position> = polygon.iter().cloned().collect();
    let mut vertices_dedup = vertices.clone();
    vertices_dedup.sort_unstable_by_key(|p| (p.x, p.y));
    vertices_dedup.dedup();

    create_pairs(&vertices_dedup)
        .par_iter()
        .filter_map(|(pos1, pos2)| {
            // Skip if not a valid rectangle
            if pos1.x == pos2.x || pos1.y == pos2.y {
                return None;
            }

            let x_start = pos1.x.min(pos2.x);
            let x_end = pos1.x.max(pos2.x);
            let y_start = pos1.y.min(pos2.y);
            let y_end = pos1.y.max(pos2.y);

            // Find compressed coordinates
            let comp_x_start = unique_x.binary_search(&x_start).ok()?;
            let comp_x_end = unique_x.binary_search(&x_end).ok()?;
            let comp_y_start = unique_y.binary_search(&y_start).ok()?;
            let comp_y_end = unique_y.binary_search(&y_end).ok()?;

            // Check if all compressed cells in the rectangle are valid
            for comp_y in comp_y_start..=comp_y_end {
                for comp_x in comp_x_start..=comp_x_end {
                    if !grid[comp_y][comp_x] {
                        return None; // Rectangle contains invalid point
                    }
                }
            }

            // Valid rectangle - compute actual area
            let width = (x_end - x_start + 1) as usize;
            let height = (y_end - y_start + 1) as usize;
            Some(width * height)
        })
        .max()
        .unwrap_or(0)
}

// Helper: Check if a point lies on a line segment
fn is_point_on_segment(point: &Position, start: Position, end: Position) -> bool {
    // Check if point is collinear and within bounds
    let cross_product =
        (point.y - start.y) * (end.x - start.x) - (point.x - start.x) * (end.y - start.y);

    if cross_product != 0 {
        return false; // Not collinear
    }

    // Check if within bounds
    let x_in_range = point.x >= start.x.min(end.x) && point.x <= start.x.max(end.x);
    let y_in_range = point.y >= start.y.min(end.y) && point.y <= start.y.max(end.y);

    x_in_range && y_in_range
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
        || part2_original(&input)
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
