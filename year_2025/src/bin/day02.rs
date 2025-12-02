use core::num;
use std::time::Duration;
use progress_timer::time_function;
use fancy_regex::Regex;

fn is_sequence_repeated(s: &str) -> bool {
    let len = s.len();
    if len % 2 != 0 {
        return false;
    }
    let half = len / 2;
    &s[0..half] == &s[half..len]
}

fn is_sequence_repeated_several(s: &str) -> bool {
    let regex = Regex::new(r"^(.+?)\1+$").unwrap();
    regex.is_match(s).unwrap_or(false)
}

fn part1(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .map(|range| {
            let mut parts = range.split('-');
            let start: usize = parts.next().unwrap().parse().unwrap();
            let end: usize = parts.next().unwrap().parse().unwrap();
            let mut sum = 0;
            for num in start..=end {
                let s = num.to_string();
                if is_sequence_repeated(&s) {
                    sum += num;
                }
            }
            sum
        })
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .map(|range| {
            let mut parts = range.split('-');
            let start: usize = parts.next().unwrap().parse().unwrap();
            let end: usize = parts.next().unwrap().parse().unwrap();
            let mut sum = 0;
            for num in start..=end {
                let s = num.to_string();

                if is_sequence_repeated_several(&s) {
                    sum += num;
                }
            }
            sum
        })
        .sum::<usize>()
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
test
input
here";
        let result = part1(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_part1_example2() {
        let input = "\
another
test
case";
        let result = part1(input);
        assert_eq!(result, 0);
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
95-115";
        let result = part2(input);
        assert_eq!(result, 99 + 111);
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
