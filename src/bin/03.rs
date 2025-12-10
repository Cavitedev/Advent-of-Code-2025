advent_of_code::solution!(3);

pub fn part_one_v1(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let joltage = joltage_bank_v1(line, 2);

        sum += joltage;
    }
    Some(sum)
}

fn joltage_bank_v1(line: &str, length: usize) -> u64 {
    // / is symbol before 0 in ascii
    let mut largest_num = vec!['/'; length];

    for char in line.chars() {
        for i in 0..largest_num.len() - 1 {
            let (left, right) = largest_num.split_at_mut(i + 1);
            let a = &mut left[i];
            let b = &mut right[0];
            if b > a {
                *a = *b;
                *b = '/';
            }
        }
        let a = &mut largest_num[length - 1];
        if char > *a {
            *a = char;
        }
    }

    largest_num
        .iter()
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

pub fn part_two_v1(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let joltage = joltage_bank_v1(line, 12);

        sum += joltage;
    }
    Some(sum)
}

#[allow(clippy::needless_range_loop)]
fn joltage_bank_v2(line: &str, length: usize) -> u64 {
    let line_chars: &[u8] = line.as_bytes();
    let mut largest_num: Vec<u8> = Vec::with_capacity(length);
    let mut check_start: usize = 0;
    let mut check_end: usize = line.len() - length;

    for _ in 0..length {
        let mut max_num = line_chars[check_start];
        let mut max_pos = check_start;
        for j in check_start + 1..=check_end {
            if line_chars[j] > max_num {
                max_num = line_chars[j];
                max_pos = j;
            }
        }
        largest_num.push(max_num);
        check_start = max_pos + 1;
        check_end += 1;
    }


    String::from_utf8(largest_num)
        .unwrap()
        .parse::<u64>()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let joltage = joltage_bank_v2(line, 2);
        sum += joltage;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let joltage = joltage_bank_v2(line, 12);
        sum += joltage;
    }
    Some(sum)
}

fn joltage_bank_v3(line: &str, length: usize) -> u64 {
    let line_chars: &[u8] = line.as_bytes();

    let mut chars_with_index: Vec<(u8, usize)> = line_chars
        .iter()
        .enumerate()
        .map(|(i, &c)| (c, i))
        .collect();

    chars_with_index.sort_by(|a, b| b.0.cmp(&a.0));

    let mut largest_num: Vec<u8> = Vec::with_capacity(length);
    let mut check_start: usize = 0;
    let mut check_end: usize = line.len() - length;

    for _ in 0..length {
        for j in 0..chars_with_index.len() {
            let max_char = chars_with_index[j];
            if max_char.1 >= check_start && max_char.1 <= check_end {
                check_start = chars_with_index[j].1;
                check_end += 1;
                largest_num.push(chars_with_index[j].0);
                chars_with_index.remove(j);
                break;
            }
        }
    }


    String::from_utf8(largest_num)
        .unwrap()
        .parse::<u64>()
        .unwrap()
}

pub fn part_one_v3(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let joltage = joltage_bank_v3(line, 2);
        sum += joltage;
    }
    Some(sum)
}

pub fn part_two_v3(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        let joltage = joltage_bank_v3(line, 12);
        sum += joltage;
    }
    Some(sum)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_v3(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two_v3(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
