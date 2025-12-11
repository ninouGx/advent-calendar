use std::{ str::FromStr, time::Duration };
use progress_timer::time_function;

use std::collections::{ HashMap, HashSet };

type NodeId = usize;

struct DAG {
    nodes: Vec<String>,
    name_to_id: HashMap<String, NodeId>,
    edges: Vec<HashSet<NodeId>>,
    reverse_edges: Vec<HashSet<NodeId>>,
}

impl DAG {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            name_to_id: HashMap::new(),
            edges: Vec::new(),
            reverse_edges: Vec::new(),
        }
    }

    fn get_or_create_node(&mut self, name: &str) -> NodeId {
        if let Some(&id) = self.name_to_id.get(name) {
            id
        } else {
            let id = self.nodes.len();
            self.nodes.push(name.to_string());
            self.name_to_id.insert(name.to_string(), id);
            self.edges.push(HashSet::new());
            self.reverse_edges.push(HashSet::new());
            id
        }
    }

    fn _add_edge(&mut self, from: &str, to: &str) {
        let from_id = self.get_or_create_node(from);
        let to_id = self.get_or_create_node(to);

        self.edges[from_id].insert(to_id);
        self.reverse_edges[to_id].insert(from_id);
    }

    fn add_edges<I>(&mut self, from: &str, to_iter: I) where I: IntoIterator, I::Item: AsRef<str> {
        let from_id = self.get_or_create_node(from);
        for to in to_iter {
            let to_id = self.get_or_create_node(to.as_ref());
            self.edges[from_id].insert(to_id);
            self.reverse_edges[to_id].insert(from_id);
        }
    }

    fn _successors(&self, node_id: NodeId) -> &HashSet<NodeId> {
        &self.edges[node_id]
    }

    fn _predecessors(&self, node_id: NodeId) -> &HashSet<NodeId> {
        &self.reverse_edges[node_id]
    }

    fn all_paths(&self, start: NodeId, end: NodeId) -> Vec<Vec<NodeId>> {
        let mut paths = Vec::new();
        let mut current_path = Vec::new();
        let mut visited = vec![false; self.nodes.len()];

        self.find_all_paths(start, end, &mut visited, &mut current_path, &mut paths);
        paths
    }

    fn find_all_paths(
        &self,
        current: NodeId,
        target: NodeId,
        visited: &mut [bool],
        path: &mut Vec<NodeId>,
        all_paths: &mut Vec<Vec<NodeId>>
    ) {
        visited[current] = true;
        path.push(current);

        if current == target {
            all_paths.push(path.clone());
        } else {
            for &neighbor in &self.edges[current] {
                if !visited[neighbor] {
                    self.find_all_paths(neighbor, target, visited, path, all_paths);
                }
            }
        }

        path.pop();
        visited[current] = false;
    }

    fn count_all_paths(
        &self,
        current: NodeId,
        target: NodeId,
        memo: &mut HashMap<NodeId, usize>
    ) -> usize {
        if current == target {
            return 1;
        }
        if let Some(&count) = memo.get(&current) {
            return count;
        }

        let mut total_paths = 0;
        for &neighbor in &self.edges[current] {
            let count = self.count_all_paths(neighbor, target, memo);
            total_paths += count;
        }

        memo.insert(current, total_paths);
        total_paths
    }

    fn total_paths(&self, start: NodeId, end: NodeId) -> usize {
        let mut memo: HashMap<NodeId, usize> = HashMap::new();
        self.count_all_paths(start, end, &mut memo)
    }
}

impl FromStr for DAG {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dag = DAG::new();

        s.lines().for_each(|line| {
            let parts: Vec<&str> = line
                .split(':')
                .map(|s| s.trim())
                .collect();
            if parts.len() == 2 {
                let from = parts[0];
                let to_nodes: Vec<&str> = parts[1].split(' ').collect();
                dag.add_edges(from, to_nodes);
            }
        });

        Ok(dag)
    }
}

fn part1(input: &str) -> usize {
    let cables = DAG::from_str(input).unwrap();
    let me = cables.name_to_id.get("you").copied().unwrap();
    let out = cables.name_to_id.get("out").copied().unwrap();
    cables.all_paths(me, out).len()
}

fn part2(input: &str) -> usize {
    let cables = DAG::from_str(input).unwrap();
    let serv = cables.name_to_id.get("svr").copied().unwrap();
    let out = cables.name_to_id.get("out").copied().unwrap();

    let digital_to_analog_converter = cables.name_to_id.get("dac").copied().unwrap();
    let fast_fourier_transform = cables.name_to_id.get("fft").copied().unwrap();

    let first_routes =
        cables.total_paths(serv, digital_to_analog_converter) *
        cables.total_paths(digital_to_analog_converter, fast_fourier_transform) *
        cables.total_paths(fast_fourier_transform, out);
    let second_routes =
        cables.total_paths(serv, fast_fourier_transform) *
        cables.total_paths(fast_fourier_transform, digital_to_analog_converter) *
        cables.total_paths(digital_to_analog_converter, out);

    first_routes + second_routes
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
you: out
"#.trim();
        let result = part1(input);
        assert_eq!(result, 1);
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
        let input =
            r#"
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
"#.trim();
        let result = part2(input);
        assert_eq!(result, 2);
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
