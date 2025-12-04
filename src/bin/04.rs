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

pub fn part_two(input: &str) -> Option<u64> {
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
