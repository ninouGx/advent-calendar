use std::time::Duration;
use progress_timer::time_function;

#[allow(dead_code)]
#[deprecated(note = "This implementation is only working for number sizes of 2")]
fn biggest_nb(numbers: &Vec<char>) -> usize {
    let last_index = numbers.len() - 1;
    let mut biggest_numbe_ten = numbers[0];
    let mut index_of_biggest = 0;
    for i in 1..last_index {
        if numbers[i] > biggest_numbe_ten {
            println!("Updating ten from {} to {}", biggest_numbe_ten, numbers[i]);
            biggest_numbe_ten = numbers[i];
            index_of_biggest = i;
        }
    }
    let mut biggest_number_unit = numbers[numbers.len() - 1];
    for i in (index_of_biggest + 1..=last_index).rev() {
        if numbers[i] > biggest_number_unit {
            println!("Updating unit from {} to {}", biggest_number_unit, numbers[i]);
            biggest_number_unit = numbers[i];
        }
    }
    let biggest_number_str = format!("{}{}", biggest_numbe_ten, biggest_number_unit);
    println!("Biggest number formed: {}", biggest_number_str);
    biggest_number_str.parse::<usize>().unwrap()
}

fn biggest_nb_x_sizes(numbers: &Vec<char>, size: usize) -> usize {
    let mut last_index = numbers.len() - size;
    let mut left_start_index = 0;
    let mut number_size_twelve: Vec<char> = vec![' '; size];
    for i in 0..size {
        let mut biggest_num = numbers[left_start_index];
        let mut index_of_biggest = left_start_index;
        for j in left_start_index..=last_index {
            if numbers[j] > biggest_num {
                biggest_num = numbers[j];
                index_of_biggest = j;
            }
        }
        number_size_twelve[i] = biggest_num;
        left_start_index = index_of_biggest + 1;
        last_index += 1;
    }

    let biggest_number_str = number_size_twelve.iter().collect::<String>();
    biggest_number_str.parse::<usize>().unwrap()
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            biggest_nb_x_sizes(&chars, 2)
        })
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let chars: Vec<char> = line.chars().collect();
            biggest_nb_x_sizes(&chars, 12)
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
121128";
        let result = part1(input);
        assert_eq!(result, 28);
    }

    #[test]
    fn test_part1_example2() {
        let input = "\
999210";
        let result = part1(input);
        assert_eq!(result, 99);
    }

    #[test]
    fn test_part2_example1() {
        let input = "\
811111111111119";
        let result = part2(input);
        assert_eq!(result, 811111111119);
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
