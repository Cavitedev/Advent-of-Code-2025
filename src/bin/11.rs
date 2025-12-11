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

pub fn part_two(input: &str) -> Option<u64> {
    let movements = parse(input);

    let mut end_paths_count: u64 = 0;
    end_paths_count += traverse_path_2("svr", &movements);
    Some(end_paths_count)
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

    #[test]
    fn test_part_two_input() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2));
    }
}
