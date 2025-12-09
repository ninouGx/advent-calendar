use std::{ str::FromStr, time::Duration };
use progress_timer::time_function;

#[derive(Debug)]
enum Rotation {
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    rotation: Rotation,
    steps: u32,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first_char = s.chars().next().ok_or("Empty string")?;
        let rotation = match first_char {
            'L' => Rotation::Left,
            'R' => Rotation::Right,
            _ => {
                return Err(format!("Invalid rotation: {}", first_char));
            }
        };
        let steps: u32 = s[1..].parse().map_err(|e| format!("Failed to parse steps: {}", e))?;
        Ok(Move { rotation, steps })
    }
}

impl Move {
    fn get_offset(&self) -> i32 {
        match self.rotation {
            Rotation::Left => -(self.steps as i32),
            Rotation::Right => self.steps as i32,
        }
    }
}

fn part1(input: &str) -> usize {
    let mut password: usize = 0;
    let moves: Vec<Move> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let mut position: i32 = 50;
    for mv in moves {
        position = (position + mv.get_offset()) % 100;
        if position == 0 {
            password += 1;
        }
    }
    password
}

#[deprecated(note = "This implementation is completely shit, see comments")]
/// # Why This Formula Is Broken (off by +2)
///
/// ## The Formula
/// ```text
/// (new_pos.div_euclid(100) - pos.div_euclid(100)).abs()
/// ```
///
/// This counts **"century boundary" crossings** (`..., -100, 0, 100, 200, ...`),
/// but we need to count **arrivals at dial position 0**, not boundary crossings.
///
/// ---
///
/// ## Analysis
///
/// ### Case 1: Crossing through 100 (correct)
/// | Start | Move | End | Formula | Expected | Result |
/// |-------|------|-----|---------|----------|--------|
/// | `50` | `R50` | `100` | `\|100/100 - 50/100\| = 1` | `1` | ✅ |
///
/// ### Case 2: Landing exactly on 0 (UNDERCOUNT)
/// | Start | Move | End | Formula | Expected | Result |
/// |-------|------|-----|---------|----------|--------|
/// | `15` | `L15` | `0` | `\|0/100 - 15/100\| = 0` | `1` | ❌ |
///
/// > We arrive at dial 0, but stay in century 0 → **not counted!**
///
/// ### Case 3: Departing from 0 going left (OVERCOUNT)
/// | Start | Move | End | Formula | Expected | Result |
/// |-------|------|-----|---------|----------|--------|
/// | `0` | `L35` | `-35` | `\|-35/100 - 0/100\| = 1` | `0` | ❌ |
///
/// > We cross into century -1, formula counts 1. But on dial: `0→99→98→...→65`.
/// > We **started** at 0 but never **arrived** at 0 during this move!
///
/// ---
///
/// ## The Problem
///
/// - **Undercounts**: Landing on `new_pos = 0` (stays in century 0)
/// - **Overcounts**: Departing from `pos = 0` going negative (crosses century boundary)
///
/// In the input, overcounts exceed undercounts by 2.
///
/// | Computed | Expected |
/// |----------|----------|
/// | `4556` | `4554` |
///
/// ---
///
/// ## Conclusion
///
/// > **The O(n) step-by-step simulation is the only truly correct approach.**
fn part2_old(input: &str) -> usize {
    let mut password: usize = 0;
    let moves: Vec<Move> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let mut position: i32 = 50;
    for mv in moves {
        let new_position = position + mv.get_offset();

        // This counts century transitions, not actual clicks at 0
        // See giant comment above for why this is subtly wrong
        let zero_crossing = (new_position.div_euclid(100) - position.div_euclid(100)).abs();

        let points_at_zero = zero_crossing;

        println!(
            "Move: {:?}, from {} to {} ({}), points at 0: {}",
            mv.get_offset(),
            position,
            new_position.rem_euclid(100),
            new_position,
            points_at_zero
        );

        password += points_at_zero as usize;
        position = new_position.rem_euclid(100);
    }
    password
}

fn part2(input: &str) -> usize {
    let mut password: usize = 0;
    let moves: Vec<Move> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let mut position: i32 = 50;
    for mv in moves {
        let old_position = position;
        let mut click_zero = 0;
        let sign = mv.get_offset().signum();
        for _ in 0..mv.steps {
            position = (position + sign).rem_euclid(100);
            if position == 0 {
                click_zero += 1;
            }
        }
        /*println!(
            "Move: {:?}, from {} to {} ({}), clicks at 0: {}",
            mv.get_offset(),
            old_position,
            position,
            old_position + mv.get_offset(),
            click_zero
        );*/
        password += click_zero as usize;
    }
    password
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
R1000
L80";
        let result = part2(input);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part2_example2() {
        let input = "\
R50
R0
L0
R100";
        let result = part2(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2_edge_case() {
        let input = "\
L50
L49
R99";
        let result = part2(input);
        assert_eq!(result, 2);
    }
}
