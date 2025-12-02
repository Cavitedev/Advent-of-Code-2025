advent_of_code::solution!(2);

fn parse<F>(input: &str, op: F) -> u64
where
    F: Fn(&str, &str) -> u64,
{
    let mut total: u64 = 0;

    for split in input.split(',') {
        let numbers: Vec<&str> = split.split('-').collect();
        total += op(numbers[0], numbers[1])
    }

    total
}

pub fn part_one(input: &str) -> Option<u64> {
    fn fun_name(num1_str: &str, num2_str: &str) -> u64 {
        let num1: u64 = num1_str.parse().unwrap();
        let num2: u64 = num2_str.parse().unwrap();
        let mut total: u64 = 0;
        let mut iterate_repeated_option: Option<u64> = None;

        if num1_str.len() % 2 == 0 {
            iterate_repeated_option = Some(
                num1_str
                    .get(0..(num1_str.len() / 2))
                    .unwrap()
                    .parse()
                    .unwrap(),
            );
        } else if num2_str.len() % 2 == 0 || num2_str.len() > num1_str.len() {
            iterate_repeated_option = Some(10u64.pow((num1_str.len() / 2) as u32));
        }

        match iterate_repeated_option {
            Some(mut iterate_repeated) => loop {
                let check_number: u64 = iterate_repeated.to_string().repeat(2).parse().unwrap();
                if check_number > num2 {
                    break;
                }
                if check_number >= num1 {
                    total += check_number;
                }
                iterate_repeated += 1;
            },
            None => {}
        }

        total
    }
    Some(parse(input, fun_name))
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(parse(input, |num1_str: &str, num2_str: &str| {
        let num1: u64 = num1_str.parse().unwrap();
        let num2: u64 = num2_str.parse().unwrap();
        let mut total: u64 = 0;
        let last_check: u64 = num2_str
            .get(0..((num2_str.len() + 1) / 2))
            .unwrap()
            .parse()
            .unwrap();

        for iterate_repeated in 1..=last_check {
            let iterate_string: String = iterate_repeated.to_string();
            if is_repeated(&iterate_string) {
                continue;
            }

            let repeat_times_start: usize = num1_str.len() / iterate_string.len();
            let repeat_times_end: usize = num2_str.len() / iterate_string.len();

            for repeat_times in repeat_times_start..=repeat_times_end {
                let check_number: u64 = iterate_string.repeat(repeat_times).parse().unwrap();
                if check_number >= num1 && check_number <= num2 {
                    total += check_number;
                }
            }
        }

        total
    }))
}

fn is_repeated(num_str: &str) -> bool {
    for num_length in 1..=num_str.len() / 2 {
        let half_number: String = num_str.get(0..num_length).unwrap().to_string();
        let full_number = half_number.repeat(num_str.len() / num_length);
        if full_number == num_str {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554u64));
    }

    #[test]
    fn test_execute_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(56660955519u64));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265u64));
    }
}
