advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum = 0;
    for line in input.lines() {
        // / is symbol before 0 in ascii
        let mut largest_num: [char; 2] = ['/'; 2];

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
            let a = &mut largest_num[largest_num.len() - 1];
            if char > *a {
                *a = char;
            }
        }
        sum += largest_num
            .iter()
            .collect::<String>()
            .parse::<u64>()
            .unwrap();
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
