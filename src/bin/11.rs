advent_of_code::solution!(11);

use std::collections::HashMap;

fn parse(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut movements: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let (start, end) = line.split_once(':').unwrap();
        let texts: Vec<&str> = end.split(' ').filter(|s| !s.is_empty()).collect();
        movements.insert(start, texts);
    }

    movements
}

fn count_paths<'a>(
    start: &'a str,
    target: &'a str,
    children: &HashMap<&'a str, Vec<&'a str>>,
    memo: &mut HashMap<(&'a str, &'a str), u64>,
) -> u64 {
    if let Some(v) = memo.get(&(start, target)) {
        return *v;
    }

    if start == target {
        memo.insert((start, target), 1);
        return 1;
    }

    let Some(ch) = children.get(start) else {
        memo.insert((start, target), 0);
        return 0;
    };

    let mut sum = 0;

    for &next in ch {
        sum += count_paths(next, target, children, memo);
    }

    memo.insert((start, target), sum);
    sum
}

pub fn part_one(input: &str) -> Option<u64> {
    let movements = parse(input);
    let mut memo = HashMap::new();
    let end_paths_count = count_paths("you", "out", &movements, &mut memo);
    Some(end_paths_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let movements: HashMap<&str, Vec<&str>> = parse(input);

    let mut memo = HashMap::new();
    let nodes_from_svr_to_fft = count_paths("svr", "fft", &movements, &mut memo);
    let nodes_from_dac_to_fft = count_paths("dac", "fft", &movements, &mut memo);

    let nodes_from_fft_to_dac = count_paths("fft", "dac", &movements, &mut memo);
    let nodes_from_svr_to_dac = count_paths("svr", "dac", &movements, &mut memo);

    let nodes_from_dac_to_out = count_paths("dac", "out", &movements, &mut memo);
    let nodes_from_fft_to_out = count_paths("fft", "out", &movements, &mut memo);

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

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(2));
    }
}
