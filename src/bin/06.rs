advent_of_code::solution!(6);
use memchr::memchr_iter;

fn run_1(input: &[u8]) -> u64 {
    let mut pos: usize = 0;
    let mut nums: Vec<Vec<u64>> = Vec::with_capacity(4);

    'outer: loop {
        let mut row_num: Vec<u64> = Vec::with_capacity(1000);
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
                row_num.push(current_num);
                break 'line;
            }
            current_num = current_num * 10 + ((byte & 0x0F) as u64);
        }
        nums.push(row_num);

        let b: u8 = input[pos];
        if b == b'+' || b == b'*' {
            break 'outer;
        }
    }

    let mut operations: Vec<u8> = Vec::with_capacity(1000);
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
    let mut sum_total = 0;

    unsafe {
        let mut grid: Vec<&[u8]> = Vec::with_capacity(5);
        let mut start = 0usize;
        for nl in memchr_iter(b'\n', input) {
            grid.push(&input[start..nl]);
            start = nl + 1;
        }

        let op_row = grid.len() - 1;
        let columns = grid[0].len();

        let mut nums = [0u64; 4];
        let mut nums_len = 0usize;

        for j in (0..columns).rev() {
            let mut current_num = 0;
            for i in 0..op_row {
                let byte = *grid.get_unchecked(i).get_unchecked(j);
                if byte == b' ' {
                    continue;
                }
                current_num = current_num * 10 + ((byte & 0x0F) as u64);
            }
            if current_num != 0 {
                nums[nums_len] = current_num;
                nums_len += 1;

                let op_byte = *grid.get_unchecked(op_row).get_unchecked(j);
                if op_byte == b' ' {
                    continue;
                } else if op_byte == b'+' {
                    for steps in 0..nums_len {
                        sum_total += nums[steps];
                    }
                    nums_len = 0;
                } else {
                    let mut product = 1;
                    for steps in 0..nums_len {
                        product *= nums[steps];
                    }
                    sum_total += product;
                    nums_len = 0;
                }
            }
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
