advent_of_code::solution!(4);
use std::cmp::min;

fn count_adjacent(grid: &[&[u8]], i: usize, j: usize) -> u8 {
    let mut count: u8 = 0;
    for i2 in i.saturating_sub(1)..min(i + 2, grid.len()) {
        for j2 in j.saturating_sub(1)..min(j + 2, grid[i].len()) {
            if i2 == i && j2 == j {
                continue;
            }
            if grid[i2][j2] == b'@' {
                count += 1;
            }
        }
    }
    count
}

pub fn part_one(input: &str) -> Option<u64> {
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();
    let mut count: u64 = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == b'@' {
                let counted_adjacent = count_adjacent(&grid, i, j);
                if counted_adjacent < 4 {
                    count += 1;
                }
            }
        }
    }
    Some(count)
}

fn count_adjacent_rounds(grid: &Vec<Vec<u8>>, i: usize, j: usize, round: u8) -> u8 {
    let mut count: u8 = 0;
    for i2 in i.saturating_sub(1)..min(i + 2, grid.len()) {
        for j2 in j.saturating_sub(1)..min(j + 2, grid[i].len()) {
            if i2 == i && j2 == j {
                continue;
            }
            if grid[i2][j2] == b'@' || (grid[i2][j2] < 20 && grid[i2][j2] >= round) {
                count += 1;
            }
        }
    }
    count
}

pub fn part_two_v1(input: &str) -> Option<u64> {
    let mut grid: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let mut last_count: u64 = 1;
    let mut count: u64 = 0;
    let mut round: u8 = 0;

    while last_count != count {
        last_count = count;
        for i in 0..grid.len() {
            for j in 0..grid[i].len() {
                if grid[i][j] == b'@' {
                    let counted_adjacent = count_adjacent_rounds(&grid, i, j, round);
                    if counted_adjacent < 4 {
                        grid[i][j] = round;
                        count += 1;
                    }
                }
            }
        }
        round += 1;
    }
    Some(count)
}

use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

#[derive(Debug)]
pub struct UniqueQueue<T> {
    queue: VecDeque<T>,
    set: HashSet<T>,
}

impl<T: Eq + Hash + Clone> UniqueQueue<T> {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            set: HashSet::new(),
        }
    }

    pub fn push(&mut self, item: T) -> bool {
        if self.set.insert(item.clone()) {
            self.queue.push_back(item);
            true
        } else {
            false
        }
    }

    pub fn push_all<I>(&mut self, items: I)
    where
        I: IntoIterator<Item = T>,
    {
        for item in items {
            self.push(item);
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(item) = self.queue.pop_front() {
            self.set.remove(&item);
            Some(item)
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

fn count_adjacent_v2(grid: &Vec<Vec<u8>>, i: usize, j: usize) -> u8 {
    let mut count: u8 = 0;
    for i2 in i.saturating_sub(1)..min(i + 2, grid.len()) {
        for j2 in j.saturating_sub(1)..min(j + 2, grid[i].len()) {
            if i2 == i && j2 == j {
                continue;
            }
            if grid[i2][j2] == b'@' {
                count += 1;
            }
        }
    }
    count
}

fn get_new_candidates(i: usize, j: usize, grid: &mut Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut offsets: Vec<(usize, usize)> = Vec::with_capacity(8);

    for i2 in i.saturating_sub(1)..min(i + 2, grid.len()) {
        for j2 in j.saturating_sub(1)..min(j + 2, grid[i2].len()) {
            if grid[i2][j2] != b'@' || i2 == i && j2 == j {
                continue;
            }
            offsets.push((i2, j2));
        }
    }
    offsets
}

fn add_adjacent(
    grid: &mut Vec<Vec<u8>>,
    count: &mut u64,
    queue: &mut UniqueQueue<(usize, usize)>,
    i: usize,
    j: usize,
) {
    if grid[i][j] == b'@' {
        let counted_adjacent = count_adjacent_v2(&grid, i, j);
        if counted_adjacent < 4 {
            grid[i][j] = b'.';
            *count += 1;
            let offsets = get_new_candidates(i, j, grid);
            queue.push_all(offsets);
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid: Vec<Vec<u8>> = input.lines().map(|line| line.as_bytes().to_vec()).collect();
    let mut count: u64 = 0;

    let mut queue: UniqueQueue<_> = UniqueQueue::new();

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            add_adjacent(&mut grid, &mut count, &mut queue, i, j);
        }
    }

    while !queue.is_empty() {
        let value = queue.pop().unwrap();
        add_adjacent(&mut grid, &mut count, &mut queue, value.0, value.1);
    }

    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
