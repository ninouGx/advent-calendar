use std::time::Duration;

struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn contains(&self, nb: usize) -> bool {
        nb >= self.start && nb <= self.end
    }

    fn is_overlapping(&self, other: &Range) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    fn is_adjacent_or_overlapping(&self, other: &Range) -> bool {
        self.end + 1 >= other.start && other.end + 1 >= self.start
    }

    fn merge(&self, other: &Range) -> Range {
        Range {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    fn size(&self) -> usize {
        self.end - self.start + 1
    }
}
use progress_timer::time_function;

fn parse_range(range: &str) -> Range {
    let mut parts = range.split('-');
    let start: usize = parts.next().unwrap().parse().unwrap();
    let end: usize = parts.next().unwrap().parse().unwrap();
    Range { start, end }
}

fn part1(input: &str) -> usize {
    let ranges = input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(parse_range)
        .collect::<Vec<Range>>();

    input
        .split("\n\n")
        .nth(1)
        .unwrap()
        .lines()
        .into_iter()
        .map(|nb| nb.parse::<usize>().unwrap())
        .filter(|&nb| { ranges.iter().any(|range| range.contains(nb)) })
        .count()
}

fn part2(input: &str) -> usize {
    let mut ranges: Vec<Range> = input
        .split("\n\n")
        .next()
        .unwrap()
        .lines()
        .map(parse_range)
        .collect();

    ranges.sort_by_key(|r| (r.start, r.end));

    ranges
        .into_iter()
        .fold(Vec::new(), |mut acc: Vec<Range>, range| {
            match acc.last_mut() {
                Some(last) if last.is_adjacent_or_overlapping(&range) => {
                    *last = last.merge(&range);
                }
                _ => acc.push(range),
            }
            acc
        })
        .into_iter()
        .map(|r| r.size())
        .sum()
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
