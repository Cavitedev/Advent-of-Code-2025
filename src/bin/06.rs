advent_of_code::solution!(6);

fn run_1(input: &[u8]) -> u64 {
    let mut pos: usize = 0;
    let mut nums: Vec<Vec<u64>> = Vec::with_capacity(4);

    'outer: loop {
        let mut row_num: Vec<u64> = Vec::with_capacity(50);
        let mut current_num = 0;
        'line: loop {
            let byte = input[pos];
            pos += 1;

            if byte == b' ' {
                if current_num > 0 {
                    row_num.push(current_num);
                    current_num = 0;
                }
                continue;
            } else if byte == b'\n' {
                if current_num > 0 {
                    row_num.push(current_num);
                }
                break 'line;
            }
            current_num = current_num * 10 + u64::from(byte - b'0');
        }
        nums.push(row_num);

        let b: u8 = input[pos];
        if b == b'+' || b == b'*' {
            break 'outer;
        }
    }

    let mut operations: Vec<u8> = Vec::with_capacity(50);
    'line: loop {
        let byte = input[pos];
        pos += 1;
        if byte == b' ' {
            continue;
        } else if byte == b'\n' {
            break 'line;
        } else {
            operations.push(byte);
        }
    }

    let mut sum_total = 0;

    for j in 0..operations.len() {
        let mut sum = nums[0][j];

        if operations[j] == b'+' {
            for i in 1..nums.len() {
                sum += nums[i][j];
            }
        } else {
            for i in 1..nums.len() {
                sum *= nums[i][j];
            }
        }
        sum_total += sum
    }

    sum_total
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(run_1(input.as_bytes()))
}

fn run_2(input: &[u8]) -> u64 {
    let grid: Vec<&[u8]> = input
        .split(|&b| b == b'\n')
        .collect();

    let mut sum_total = 0;

    let mut nums: Vec<u64> = Vec::with_capacity(4);

    for j in (0..grid[0].len()).rev() {
        let mut current_num = 0;
        for i in 0..grid.len() - 2 {
            let byte = grid[i][j];
            if byte == b' ' {
                continue;
            }
            current_num = current_num * 10 + u64::from(byte - b'0');
        }
        // Check if it may be 0 on large input
        if current_num != 0 {
            nums.push(current_num);
        }

        let byte = grid[grid.len() - 2][j];
        if byte == b' ' {
            continue;
        } else if byte == b'+' {
            sum_total += nums.iter().sum::<u64>();
            nums.clear();
        } else {
            sum_total += nums.iter().product::<u64>();
            nums.clear();
        }
    }

    sum_total
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(run_2(input.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}
