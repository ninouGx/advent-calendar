use std::time::Duration;
use progress_timer::time_function;

fn part1(input: &str) -> usize {
    0
}

fn part2(input: &str) -> usize {
    0
}

fn main() {
    let is_test = true;
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
test
input
here
"#.trim();
        let result = part1(input);
        assert_eq!(result, 0);
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
part2
test
input
"#.trim();
        let result = part2(input);
        assert_eq!(result, 0);
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
