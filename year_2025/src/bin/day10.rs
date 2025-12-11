use std::{ str::FromStr, time::Duration };
use progress_timer::time_function;
use good_lp::*;

struct Machine {
    wanted_lights: usize,
    buttons: Vec<usize>,
    voltages: Vec<usize>,
    ans: Vec<usize>,
}

impl Machine {
    fn compute_min_presses_for_xor(&self, num: usize) -> Option<usize> {
        let max_xor = (1 << self.ans.len()) - 1;
        let mut dp = vec![usize::MAX; max_xor + 1];
        dp[0] = 0;

        for &button in &self.buttons {
            let button_idx = button as usize;
            for j in (0..max_xor).rev() {
                if dp[j] != usize::MAX {
                    let new_xor = j ^ button_idx;
                    if new_xor <= max_xor {
                        dp[new_xor] = dp[new_xor].min(dp[j] + 1);
                    }
                }
            }
        }

        let target_idx = num as usize;
        if dp[target_idx] == usize::MAX {
            None
        } else {
            Some(dp[target_idx])
        }
    }

    // Need to install
    fn min_presses_ilp(&self, target: &Vec<usize>) -> Option<usize> {
        unsafe {
            std::env::set_var("CBC_MESSAGE_LEVEL", "0");
        }

        let buttons = &self.buttons;
        let n_bits = target.len();
        let n_buttons = buttons.len();

        let mut vars = ProblemVariables::new();
        let x = vars.add_vector(variable().integer().min(0), n_buttons);

        let mut objective = Expression::from(0);
        for i in 0..n_buttons {
            objective += x[i];
        }

        let mut problem = vars.minimise(objective).using(default_solver);

        for bit in 0..n_bits {
            let mut constraint_expr = Expression::from(0);

            for (j, &button) in buttons.iter().enumerate() {
                let bit_position = n_bits - 1 - bit;
                if (button & (1 << bit_position)) != 0 {
                    constraint_expr += x[j];
                }
            }

            problem = problem.with(constraint!(constraint_expr == (target[bit] as i32)));
        }

        match problem.solve() {
            Ok(solution) => {
                let total: usize = (0..n_buttons).map(|i| solution.value(x[i]) as usize).sum();
                Some(total)
            }
            Err(_) => None,
        }
    }

    fn compute_xor_button_presses(&mut self) -> Option<usize> {
        self.compute_min_presses_for_xor(self.wanted_lights)
    }

    fn compute_sum_button_presses(&mut self) -> Option<usize> {
        self.min_presses_ilp(&self.voltages)
    }
}

impl FromStr for Machine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elems = s.split_ascii_whitespace().collect::<Vec<&str>>();
        let wanted_binary = elems[0]
            .chars()
            .filter_map(|c| {
                match c {
                    '#' => Some('1'),
                    '.' => Some('0'),
                    _ => None,
                }
            })
            .collect::<String>();
        let wanted_lights = usize::from_str_radix(&wanted_binary, 2).map_err(|_| ())?;
        let stripped = elems[elems.len() - 1]
            .strip_prefix('{')
            .and_then(|s| s.strip_suffix('}'))
            .unwrap_or(elems[elems.len() - 1]);
        let voltages: Vec<usize> = stripped
            .split(',')
            .filter_map(|num_str| num_str.parse::<usize>().ok())
            .collect();
        let buttons = elems[1..elems.len() - 1]
            .iter()
            .map(|&s| {
                let stripped = s
                    .strip_prefix('(')
                    .and_then(|s| s.strip_suffix(')'))
                    .unwrap_or(s);
                stripped
                    .split(',')
                    .filter_map(|num_str| num_str.parse::<u8>().ok())
                    .collect::<Vec<u8>>()
            })
            .map(|toggles| {
                let mut button_value: usize = 0;
                let max_size = wanted_binary.len() as u8;
                for &toggle in &toggles {
                    button_value |= 1 << (max_size - 1 - toggle);
                }
                button_value
            })
            .collect::<Vec<usize>>();
        if wanted_binary.len() != voltages.len() {
            return Err(());
        }
        Ok(Machine {
            wanted_lights,
            buttons,
            ans: vec![0; 16],
            voltages,
        })
    }
}

fn part1(input: &str) -> usize {
    let machines: Vec<Machine> = input
        .lines()
        .map(|line| Machine::from_str(line).unwrap())
        .collect();
    machines
        .into_iter()
        .filter_map(|mut machine| {
            let presses = machine.compute_xor_button_presses();
            if let Some(count) = presses {
                Some(count)
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let machines: Vec<Machine> = input
        .lines()
        .map(|line| Machine::from_str(line).unwrap())
        .collect();
    machines
        .into_iter()
        .filter_map(|mut machine| {
            let presses = machine.compute_sum_button_presses();
            if let Some(count) = presses {
                Some(count)
            } else {
                None
            }
        })
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
        let input = r#"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
"#.trim();
        let result = part1(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part1_example2() {
        let input =
            r#"
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"#.trim();
        let result = part1(input);
        assert_eq!(result, 3 + 2);
    }

    #[test]
    fn test_part2_example1() {
        let input = r#"
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
"#.trim();
        let result = part2(input);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_part2_example2() {
        let input = r#"
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
"#.trim();
        let result = part2(input);
        assert_eq!(result, 12);
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
