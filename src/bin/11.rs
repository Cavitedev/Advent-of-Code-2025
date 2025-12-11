use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(11);

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut movements: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let (start, end) = line.split_once(':').unwrap();
        let texts: Vec<&str> = end.split(' ').filter(|s| !s.is_empty()).collect();
        movements.insert(start, texts);
    }

    movements
}

fn traverse_path(start: &str, movements: &HashMap<&str, Vec<&str>>) -> u64 {
    let mut ending_paths = 0;

    for next_step in movements.get(start).unwrap() {
        if *next_step == "out" {
            ending_paths += 1;
        } else {
            ending_paths += traverse_path(&next_step, &movements);
        }
    }

    ending_paths
}

pub fn part_one(input: &str) -> Option<u64> {
    let movements = parse(input);

    let mut end_paths_count: u64 = 0;

    end_paths_count += traverse_path("you", &movements);

    Some(end_paths_count)
}

fn traverse_path_2(start: &str, movements: &HashMap<&str, Vec<&str>>) -> u64 {
    let mut ending_paths = 0;

    let visited: HashSet<&str> = HashSet::new();
    let mut queue: VecDeque<(&str, bool, bool, HashSet<&str>)> = VecDeque::new();

    queue.push_back((start, false, false, visited));

    while let Some((state, has_dac, has_fft, mut visited)) = queue.pop_back() {
        visited.insert(state);
        for next_step in movements.get(state).unwrap() {
            if visited.contains(next_step) {
                continue;
            }

            if *next_step == "out" {
                if has_dac && has_fft {
                    ending_paths += 1;
                }
            } else if *next_step == "dac" {
                queue.push_back((next_step, true, has_fft, visited.clone()));
            } else if *next_step == "fft" {
                queue.push_back((&next_step, has_dac, true, visited.clone()));
            } else {
                queue.push_back((&next_step, has_dac, has_fft, visited.clone()));
            }
        }
        dbg!(visited.len());
        dbg!(queue.len());
    }

    ending_paths
}

#[derive(Debug, Clone)]
struct InversedNode<'a> {
    parents: Vec<&'a str>,
    count_out: u128,
}

fn propagate_paths_to<'a>(inversed: &mut HashMap<&'a str, InversedNode<'a>>, out: &'a str) {
    let mut queue = VecDeque::new();

    let out_node = inversed.get_mut(out).unwrap();
    out_node.count_out = 1;
    queue.push_back(out);

    // Reverse-graph DP propagation
    while let Some(node_id) = queue.pop_front() {
        let current_paths = inversed[node_id].count_out;

        // Propagate into parents
        let parents: Vec<&str> = inversed[node_id].parents.clone();

        for parent_id in parents {
            let parent = inversed.get_mut(parent_id).unwrap();
            let prev = parent.count_out;

            parent.count_out += current_paths;

            if parent.count_out != prev {
                queue.push_back(parent_id);
            }
        }
    }
}

pub fn compute_paths<'a>(inversed: &mut HashMap<&'a str, InversedNode<'a>>, out: &'a str) {
    // Build children map (forward graph)
    let mut children: HashMap<&'a str, Vec<&'a str>> = HashMap::new();
    for (&node, inv) in inversed.iter() {
        for &parent in &inv.parents {
            children.entry(parent).or_insert_with(Vec::new).push(node);
        }
    }

    // Recursively compute paths toward OUT
    fn dfs<'a>(
        node: &'a str,
        out: &'a str,
        inversed: &mut HashMap<&'a str, InversedNode<'a>>,
        children: &HashMap<&'a str, Vec<&'a str>>,
    ) -> u128 {
        // Memo?
        if inversed[node].count_out > 0 {
            return inversed[node].count_out;
        }

        if node == out {
            inversed.get_mut(node).unwrap().count_out = 1;
            return 1;
        }

        // If no outgoing edges → dead end
        let Some(ch) = children.get(node) else {
            inversed.get_mut(node).unwrap().count_out = 0;
            return 0;
        };

        let mut sum = 0;
        for &child in ch {
            sum += dfs(child, out, inversed, children);
        }

        inversed.get_mut(node).unwrap().count_out = sum;
        sum
    }

    // Compute for all nodes
    let keys: Vec<&str> = inversed.keys().copied().collect();
    for key in keys {
        dbg!(&key);
        dfs(key, out, inversed, &children);
    }
}

pub fn part_two(input: &str) -> Option<u128> {
    let movements: HashMap<&str, Vec<&str>> = parse(input);

    let mut inverse_nodes: HashMap<&str, InversedNode> = HashMap::with_capacity(movements.len());

    // let mut nodes: HashMap<&str, InversedNode> = HashMap::with_capacity(movements.len());

    // for (origin, dests) in &movements {
    //     nodes.entry(origin).or_insert(InversedNode {
    //         parents: dests.clone(),
    //         count_out: 0,
    //     });
    // }

    // dbg!(&nodes);

    for (origin, dests) in &movements {
        inverse_nodes.entry(origin).or_insert(InversedNode {
            parents: Vec::new(),
            count_out: 0,
        });

        for dest in dests {
            inverse_nodes
                .entry(dest)
                .or_insert(InversedNode {
                    parents: Vec::new(),
                    count_out: 0,
                })
                .parents
                .push(origin);
        }
    }

    let mut inverse_nodes_out = inverse_nodes.clone();
    compute_paths(&mut inverse_nodes_out, "out");
    dbg!("1");

    let mut inverse_nodes_dac = inverse_nodes.clone();
    compute_paths(&mut inverse_nodes_dac, "dac");
    dbg!("2");

    let mut inverse_nodes_fft = inverse_nodes.clone();
    compute_paths(&mut inverse_nodes_fft, "fft");
    dbg!("3");
    let nodes_from_svr_to_fft: u128 = inverse_nodes_fft.get("svr").unwrap().count_out as u128;
    let nodes_from_fft_to_dac: u128 = inverse_nodes_dac.get("fft").unwrap().count_out as u128;
    let nodes_from_dac_to_out: u128 = inverse_nodes_out.get("dac").unwrap().count_out as u128;

    let nodes_from_svr_to_dac: u128 = inverse_nodes_dac.get("svr").unwrap().count_out as u128;
    let nodes_from_dac_to_fft: u128 = inverse_nodes_fft.get("dac").unwrap().count_out as u128;
    let nodes_from_fft_to_out: u128 = inverse_nodes_out.get("fft").unwrap().count_out as u128;

    Some(
        nodes_from_svr_to_fft * nodes_from_fft_to_dac * nodes_from_dac_to_out
            + nodes_from_svr_to_dac * nodes_from_dac_to_fft * nodes_from_fft_to_out,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    // #[test]
    // fn test_part_one_input() {
    //     let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
    //     assert_eq!(result, Some(5));
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two_ex3() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two_input() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2));
    }

    // 347407494966
    // 602669810592 too low
    // 86719961286 too low
}
