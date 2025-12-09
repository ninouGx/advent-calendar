use std::str::FromStr;
use std::time::Duration;
use progress_timer::time_function;
use aoc_utils::Grid;

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
