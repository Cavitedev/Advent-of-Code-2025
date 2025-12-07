advent_of_code::solution!(7);
use memchr::memchr_iter;

fn run_01(input: &[u8]) -> u64 {
    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(150);
    let mut start = 0usize;

    for nl in memchr_iter(b'\n', input) {
        grid.push(input[start..nl].to_vec());
        start = nl + 1;
    }

    let mut count: u64 = 0;

    //Bean on S

    let index_s = grid[0].iter().position(|&c| c == b'S').unwrap();
    grid[1][index_s] = b'|';

    for i in 1..grid.len() - 1 {
        for j in 0..grid[i].len() {
            let next_row = i + 1;
            let last_byte = grid[i][j];
            if last_byte == b'|' {
                {
                    match grid[next_row][j] {
                        b'.' => grid[next_row][j] = b'|',
                        b'|' => {}
                        b'^' => {
                            count += 1;
                            propagate_bean(&mut grid, j - 1, next_row);
                            propagate_bean(&mut grid, j + 1, next_row);
                        }
                        _ => panic!("Invalid turn"),
                    }
                }
            }
        }
    }

    count
}

fn propagate_bean(grid: &mut Vec<Vec<u8>>, j: usize, next_row: usize) {
    let byte = grid[next_row][j];

    match byte {
        b'.' => {
            grid[next_row][j] = b'|';
        }
        _ => {}
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(run_01(input.as_bytes()))
}

fn run_02(input: &[u8]) -> u64 {
    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(150);
    let mut start = 0usize;

    for nl in memchr_iter(b'\n', input) {
        grid.push(input[start..nl].to_vec());
        start = nl + 1;
    }

    //Bean on S

    let index_s = grid[0].iter().position(|&c| c == b'S').unwrap();
    let mut posibilites = vec![0; grid[0].len()];
    posibilites[index_s] = 1;
    grid[1][index_s] = b'|';

    for i in 1..grid.len() - 1 {
        for j in 0..grid[i].len() {
            let next_row = i + 1;
            let last_byte = grid[i][j];
            if last_byte == b'|' {
                {
                    match grid[next_row][j] {
                        b'.' => grid[next_row][j] = b'|',
                        b'|' => {}
                        b'^' => {
                            propagate_bean(&mut grid, j - 1, next_row);
                            propagate_bean(&mut grid, j + 1, next_row);
                            posibilites[j - 1] += posibilites[j];
                            posibilites[j + 1] += posibilites[j];
                            posibilites[j] = 0;
                        }
                        _ => panic!("Invalid turn"),
                    }
                }
            }
        }
    }

    posibilites.iter().sum()
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(run_02(input.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}
