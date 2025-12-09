use std::{ collections::{ HashMap, HashSet }, time::Duration };
use progress_timer::time_function;
use aoc_utils::{ Position, Direction, display_grid_animated };

fn part1(input: &str) -> usize {
    let splitter_hashset: std::collections::HashSet<Position> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, ch)| {
                    if ch == '^' {
                        Some(Position {
                            x: x as i32,
                            y: y as i32,
                        })
                    } else {
                        None
                    }
                })
        })
        .collect();

    let mut laser_cache: HashMap<usize, HashSet<Position>> = input
        .chars()
        .position(|c| c == 'S')
        .map(|index| {
            let mut map: HashMap<usize, HashSet<Position>> = HashMap::new();
            map.insert(
                0,
                vec![Position {
                    x: index as i32,
                    y: 0,
                }]
                    .into_iter()
                    .collect()
            );
            map
        })
        .unwrap_or_default();

    let height = input.lines().count() as i32;

    let mut nb_split = 0;
    (0..height).for_each(|step| {
        let step = step as usize;
        let all_lasers: HashSet<Position> = laser_cache.values().flatten().cloned().collect();
        display_grid_animated(
            [
                (&splitter_hashset, '^'),
                (&all_lasers, '|'),
            ],
            step,
            500
        );
        let positions: Vec<Position> = laser_cache
            .get(&step)
            .unwrap_or(&HashSet::new())
            .iter()
            .cloned()
            .collect();
        for pos in positions {
            if splitter_hashset.contains(&pos.get_adjacent(&Direction::South)) {
                nb_split += 1;
                laser_cache
                    .entry(step + 1)
                    .or_default()
                    .insert(pos.get_adjacent(&Direction::SouthEast));
                laser_cache
                    .entry(step + 1)
                    .or_default()
                    .insert(pos.get_adjacent(&Direction::SouthWest));
            } else {
                laser_cache
                    .entry(step + 1)
                    .or_default()
                    .insert(pos.get_adjacent(&Direction::South));
            }
        }
    });

    nb_split
}

fn part2(input: &str) -> usize {
    let splitter_hashset: std::collections::HashSet<Position> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, ch)| {
                    if ch == '^' {
                        Some(Position {
                            x: x as i32,
                            y: y as i32,
                        })
                    } else {
                        None
                    }
                })
        })
        .collect();

    let mut laser_cache: HashMap<usize, HashMap<Position, usize>> = input
        .chars()
        .position(|c| c == 'S')
        .map(|index| {
            let mut map: HashMap<usize, HashMap<Position, usize>> = HashMap::new();
            let mut inner_map: HashMap<Position, usize> = HashMap::new();
            inner_map.insert(
                Position {
                    x: index as i32,
                    y: 0,
                },
                1
            );
            map.insert(0, inner_map);
            map
        })
        .unwrap_or_default();

    let height = input.lines().count() as i32;

    (0..height).for_each(|step| {
        let step = step as usize;
        //display_grid_step(&splitter_hashset, &laser_cache, step);
        let positions: Vec<Position> = laser_cache
            .get(&step)
            .unwrap_or(&HashMap::new())
            .keys()
            .cloned()
            .collect();

        for pos in positions {
            let current_paths = laser_cache.get(&step).unwrap().get(&pos).cloned().unwrap_or(0);
            if splitter_hashset.contains(&pos.get_adjacent(&Direction::South)) {
                let south_east = pos.get_adjacent(&Direction::SouthEast);
                let south_west = pos.get_adjacent(&Direction::SouthWest);

                let se_nb = laser_cache
                    .get(&(step + 1))
                    .and_then(|m| m.get(&south_east))
                    .cloned()
                    .unwrap_or(0);
                laser_cache
                    .entry(step + 1)
                    .or_default()
                    .insert(south_east, se_nb + current_paths);

                let sw_nb = laser_cache
                    .get(&(step + 1))
                    .and_then(|m| m.get(&south_west))
                    .cloned()
                    .unwrap_or(0);
                laser_cache
                    .entry(step + 1)
                    .or_default()
                    .insert(south_west, sw_nb + current_paths);
            } else {
                let south = pos.get_adjacent(&Direction::South);
                let nb_s = laser_cache
                    .get(&(step + 1))
                    .and_then(|m| m.get(&south))
                    .cloned()
                    .unwrap_or(0);
                laser_cache
                    .entry(step + 1)
                    .or_default()
                    .insert(south, nb_s + current_paths);
            }
        }
    });

    laser_cache
        .get(&input.lines().count())
        .map(|map| map.values().sum())
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
^S.
...
.^.
"#.trim();
        let result = part1(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part1_example2() {
        let input = r#"
..S.
....
..^.
.^..
....
"#.trim();
        let result = part1(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2_example1() {
        let input = r#"
..S.
....
..^.
.^..
....
"#.trim();
        let result = part2(input);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part2_example2() {
        let input = r#"
..S.
....
..^.
.^.^
^...
"#.trim();
        let result = part2(input);
        assert_eq!(result, 5);
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
