advent_of_code::solution!(8);
use std::{cmp::Ordering, collections::HashSet};

#[derive(PartialEq, PartialOrd)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn from_line(line: &str) -> Point {
        let positions: Vec<i64> = line
            .split(",")
            .map(|values| values.parse::<i64>().unwrap())
            .collect();
        Point {
            x: positions[0],
            y: positions[1],
            z: positions[2],
        }
    }
}

fn parse_input(input: &str) -> Vec<Point> {
    let mut vec = Vec::with_capacity(1000);

    for line in input.lines() {
        vec.push(Point::from_line(line));
    }

    vec
}

#[derive(PartialEq, Eq)]
struct Distance {
    start_idx: usize,
    end_idx: usize,
    distance: i64,
}

impl PartialOrd for Distance {
    #[allow(clippy::non_canonical_partial_ord_impl)]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.distance.cmp(&other.distance))
    }
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.cmp(&other.distance)
    }
}

fn calculate_sqr_dis(a: &Point, b: &Point) -> i64 {
    (a.x - b.x).pow(2) + (a.y - b.y).pow(2) + (a.z - b.z).pow(2)
}

//Check later how to return less elements
fn ordered_shortest_distances(points: &[Point]) -> Vec<Distance> {
    let mut distances: Vec<Distance> = Vec::with_capacity(10000);

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let dis = calculate_sqr_dis(&points[i], &points[j]);
            distances.push(Distance {
                start_idx: i,
                end_idx: j,
                distance: dis,
            });
        }
    }
    distances.sort();

    distances
}

fn find_grid(grids: &[HashSet<usize>], a: usize, b: usize) -> (Option<usize>, Option<usize>) {
    let mut option1: Option<usize> = None;
    let mut option2: Option<usize> = None;

    for (i, grid) in grids.iter().enumerate() {
        if grid.contains(&a) {
            option1 = Some(i);
        }
        if grid.contains(&b) {
            option2 = Some(i);
        }
    }
    (option1, option2)
}

fn run_1_with_count(input: &str, count: usize) -> usize {
    let parsed_input = parse_input(input);
    let shortest_distances = ordered_shortest_distances(&parsed_input);

    let mut grids: Vec<HashSet<usize>> = Vec::with_capacity(5000);

    for shortest_dis in shortest_distances.iter().take(count) {
        let connect_grids_idx = find_grid(&grids, shortest_dis.start_idx, shortest_dis.end_idx);
        if connect_grids_idx.0.is_none() && connect_grids_idx.1.is_none() {
            let mut grid: HashSet<usize> = HashSet::new();
            grid.insert(shortest_dis.start_idx);
            grid.insert(shortest_dis.end_idx);
            grids.push(grid);
        } else if connect_grids_idx.0.is_some() && connect_grids_idx.1.is_some() {
            let i = connect_grids_idx.0.unwrap();
            let j = connect_grids_idx.1.unwrap();
            if i == j {
                continue;
            }

            let (small, large) = if i < j { (i, j) } else { (j, i) };

            let moved = grids.remove(large);
            grids[small].extend(moved);
        } else if connect_grids_idx.0.is_some() {
            let grid_id = connect_grids_idx.0.unwrap();
            grids[grid_id].insert(shortest_dis.end_idx);
        } else {
            let grid_id = connect_grids_idx.1.unwrap();
            grids[grid_id].insert(shortest_dis.start_idx);
        }
    }

    grids.sort_by_key(|b| std::cmp::Reverse(b.len()));
    grids[0].len() * grids[1].len() * grids[2].len()
}

pub fn run_1(input: &str) -> usize {
    run_1_with_count(input, 1000)
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(run_1(input))
}

pub fn run_2(input: &str) -> i64 {
    let points = parse_input(input);
    let shortest_distances = ordered_shortest_distances(&points);

    let mut grids: Vec<HashSet<usize>> = Vec::with_capacity(5000);

    for shortest_dis in shortest_distances {
        let connect_grids_idx = find_grid(&grids, shortest_dis.start_idx, shortest_dis.end_idx);
        if connect_grids_idx.0.is_none() && connect_grids_idx.1.is_none() {
            let mut grid: HashSet<usize> = HashSet::new();
            grid.insert(shortest_dis.start_idx);
            grid.insert(shortest_dis.end_idx);
            grids.push(grid);
        } else if connect_grids_idx.0.is_some() && connect_grids_idx.1.is_some() {
            let i = connect_grids_idx.0.unwrap();
            let j = connect_grids_idx.1.unwrap();
            if i == j {
                continue;
            }

            let (small, large) = if i < j { (i, j) } else { (j, i) };

            let moved = grids.remove(large);
            grids[small].extend(moved);
        } else if connect_grids_idx.0.is_some() {
            let grid_id = connect_grids_idx.0.unwrap();
            grids[grid_id].insert(shortest_dis.end_idx);
        } else {
            let grid_id = connect_grids_idx.1.unwrap();
            grids[grid_id].insert(shortest_dis.start_idx);
        }
        if grids[0].len() == points.len() {
            let point_start = &points[shortest_dis.start_idx];
            let point_end = &points[shortest_dis.end_idx];
            return point_start.x * point_end.x;
        }
    }

    0
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(run_2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = run_1_with_count(&advent_of_code::template::read_file("examples", DAY), 10);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}
