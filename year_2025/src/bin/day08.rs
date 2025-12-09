use std::{ cmp::Reverse, collections::HashSet, fmt::Display, str::FromStr, time::Duration };
use progress_timer::time_function;
use aoc_utils::create_pairs;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

impl Coordinate {
    fn euclidean_distance(&self, other: &Coordinate) -> f64 {
        let dx = (self.x - other.x) as f64;
        let dy = (self.y - other.y) as f64;
        let dz = (self.z - other.z) as f64;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

impl FromStr for Coordinate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split(',').collect();
        if parts.len() != 3 {
            return Err(format!("Invalid coordinate format: {}", s));
        }
        let x = parts[0].parse::<i32>().map_err(|e| format!("Failed to parse x: {}", e))?;
        let y = parts[1].parse::<i32>().map_err(|e| format!("Failed to parse y: {}", e))?;
        let z = parts[2].parse::<i32>().map_err(|e| format!("Failed to parse z: {}", e))?;
        Ok(Coordinate { x, y, z })
    }
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

fn part1(input: &str) -> usize {
    let coordinates: Vec<Coordinate> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let mut sorted = create_pairs(&coordinates);
    sorted.sort_by(|(a, b), (c, d)| {
        let dist1 = a.euclidean_distance(b);
        let dist2 = c.euclidean_distance(d);
        dist1.partial_cmp(&dist2).unwrap()
    });

    let mut circuits: Vec<HashSet<Coordinate>> = Vec::new();
    let nb_pairs_to_consider = 1000;
    for (coord1, coord2) in sorted.into_iter().take(nb_pairs_to_consider) {
        let idx1 = circuits.iter().position(|c| c.contains(coord1));
        let idx2 = circuits.iter().position(|c| c.contains(coord2));

        match (idx1, idx2) {
            (Some(i), Some(j)) if i != j => {
                let circuit_j = std::mem::take(&mut circuits[j]);
                circuits[i].extend(circuit_j);
            }
            (Some(_i), Some(_j)) => {}
            (Some(i), None) => {
                circuits[i].insert(coord2.clone());
            }
            (None, Some(j)) => {
                circuits[j].insert(coord1.clone());
            }
            (None, None) => {
                let mut new_circuit = HashSet::new();
                new_circuit.insert(coord1.clone());
                new_circuit.insert(coord2.clone());
                circuits.push(new_circuit);
            }
        }
    }
    circuits.retain(|c| !c.is_empty());

    circuits.sort_by_key(|circuit| Reverse(circuit.len()));

    circuits
        .iter()
        .take(3)
        .map(|circuit| circuit.len())
        .product()
}

fn part2(input: &str) -> usize {
    let coordinates: Vec<Coordinate> = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    let coords_len = coordinates.len();
    let mut sorted = create_pairs(&coordinates);
    sorted.sort_by(|(a, b), (c, d)| {
        let dist1 = a.euclidean_distance(b);
        let dist2 = c.euclidean_distance(d);
        dist1.partial_cmp(&dist2).unwrap()
    });

    let mut global_making_pair: Option<(Coordinate, Coordinate)> = None;
    let mut circuits: Vec<HashSet<Coordinate>> = Vec::new();
    for (coord1, coord2) in sorted.into_iter() {
        let idx1 = circuits.iter().position(|c| c.contains(coord1));
        let idx2 = circuits.iter().position(|c| c.contains(coord2));

        match (idx1, idx2) {
            (Some(i), Some(j)) if i != j => {
                let circuit_j = std::mem::take(&mut circuits[j]);
                circuits[i].extend(circuit_j);
            }
            (Some(_i), Some(_j)) => {}
            (Some(i), None) => {
                circuits[i].insert(coord2.clone());
            }
            (None, Some(j)) => {
                circuits[j].insert(coord1.clone());
            }
            (None, None) => {
                let mut new_circuit = HashSet::new();
                new_circuit.insert(coord1.clone());
                new_circuit.insert(coord2.clone());
                circuits.push(new_circuit);
            }
        }

        if
            circuits
                .iter()
                .map(|c| c.len())
                .sum::<usize>() == coords_len
        {
            global_making_pair = Some((coord1.clone(), coord2.clone()));
            break;
        }
    }
    match global_making_pair {
        Some((c1, c2)) => (c1.x as usize) * (c2.x as usize),
        None => 0,
    }
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
162,817,812
425,690,689
"#.trim();
        let result = part1(input);
        assert_eq!(result, 2 * 1 * 1);
    }

    #[test]
    fn test_part1_example2() {
        let input = r#"
162,817,812
425,690,689
431,825,988
906,360,560
805,96,715
"#.trim();
        let result = part1(input);
        assert_eq!(result, 3 * 2 * 1);
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
