use std::{ collections::HashMap, time::Duration };
use progress_timer::time_function;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    fn from_str(s: &str) -> Option<Operator> {
        match s {
            "+" => Some(Operator::Add),
            "-" => Some(Operator::Subtract),
            "*" => Some(Operator::Multiply),
            "/" => Some(Operator::Divide),
            _ => None,
        }
    }
}

struct Problem {
    numbersCache: std::collections::HashMap<usize, Vec<usize>>,
    operatorCache: std::collections::HashMap<usize, Operator>,
    sizeCache: std::collections::HashMap<usize, usize>,
}

impl Problem {
    fn add_number(&mut self, id: usize, number: usize) {
        self.numbersCache.entry(id).or_default().push(number);
    }

    fn set_operator(&mut self, id: usize, operator: Operator) {
        self.operatorCache.insert(id, operator);
    }

    fn solve(&self) -> usize {
        self.operatorCache
            .iter()
            .map(|(&id, &op)| {
                let binding = Vec::new();
                let numbers = self.numbersCache.get(&id).unwrap_or(&binding);
                do_the_math(op, numbers)
            })
            .sum()
    }
}

fn do_the_math(op: Operator, nbs: &Vec<usize>) -> usize {
    match op {
        Operator::Add => nbs.iter().sum(),
        Operator::Subtract => nbs.iter().fold(0, |acc, &x| acc - x),
        Operator::Multiply => nbs.iter().product(),
        Operator::Divide => nbs.iter().fold(1, |acc, &x| acc / x),
    }
}

fn count_nb_whitespaces(s: &str) -> usize {
    s.chars()
        .filter(|c| c.is_whitespace())
        .count()
}

fn part1(input: &str) -> usize {
    let mut pb = Problem {
        numbersCache: std::collections::HashMap::new(),
        operatorCache: std::collections::HashMap::new(),
        sizeCache: std::collections::HashMap::new(),
    };
    input.lines().for_each(|line| {
        line.split_ascii_whitespace()
            .enumerate()
            .for_each(|(index, elem)| {
                if let Some(op) = Operator::from_str(elem) {
                    pb.set_operator(index, op);
                } else if let Ok(num) = elem.parse::<usize>() {
                    pb.add_number(index, num);
                }
            })
    });

    pb.solve()
}

fn part2(input: &str) -> usize {
    let mut pb = Problem {
        numbersCache: std::collections::HashMap::new(),
        operatorCache: std::collections::HashMap::new(),
        sizeCache: std::collections::HashMap::new(),
    };

    let chars: Vec<char> = input.lines().last().unwrap().chars().collect();
    let mut prev_index: Option<usize> = None;

    chars
        .iter()
        .enumerate()
        .for_each(|(index, &elem)| {
            if let Some(op) = Operator::from_str(&elem.to_string()) {
                if let Some(prev) = prev_index {
                    let size = index - prev;
                    pb.sizeCache.insert(prev, size);
                }
                pb.set_operator(index, op);
                prev_index = Some(index);
            }
        });
    if let Some(prev) = prev_index {
        let size = chars.len() - prev;
        pb.sizeCache.insert(prev, size);
    }

    // Remove last line which contains operators
    let input_without_op = input
        .rsplit_once('\n')
        .map(|(rest, _last)| rest)
        .unwrap_or(input)
        .lines();

    // Collect sizeCache to avoid borrowing conflict
    let size_entries: Vec<(usize, usize)> = pb.sizeCache
        .iter()
        .map(|(&k, &v)| (k, v))
        .collect();

    // Iterate over each operator position to create numbers from columns
    size_entries.iter().for_each(|&(op_index, size)| {
        (op_index..op_index + size).for_each(|col_index| {
            let vertical_number = input_without_op
                .clone()
                .filter_map(|line| { line.chars().nth(col_index) })

                .collect::<String>()
                .trim()
                .to_string();
            if let Ok(num) = vertical_number.parse::<usize>() {
                pb.add_number(op_index, num);
            }
        });
    });

    pb.solve()
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
64 
23 
314
+  ";
        let result = part2(input);
        assert_eq!(result, 623 + 431 + 4);
    }

    #[test]
    fn test_part2_example2() {
        let input = r#"
  1
 42
869
*  "#;
        let result = part2(input);
        assert_eq!(result, 8 * 46 * 129);
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
