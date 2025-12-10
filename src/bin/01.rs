advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut current_value: i64 = 50;
    let mut total_count = 0;
    for line in input.lines() {
        let (turn, value) = line.split_at(1);
        let move_value: i64 = value.parse().unwrap();
        current_value = turn_wheel(turn, move_value, &current_value);
        current_value = current_value.rem_euclid(100i64);
        if current_value == 0 {
            total_count += 1
        }
    }
    Some(total_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut current_value: i64 = 50;
    let mut total_count: u64 = 0;
    input.lines().for_each(|line| {
        let (turn, value) = line.split_at(1);
        let move_value: i64 = value.parse().unwrap();
        let last_value: i64 = current_value;
        current_value = turn_wheel(turn, move_value, &current_value);
        if (last_value > 0) != (current_value > 0) && last_value != 0 {
            total_count += 1
        }
        total_count += current_value.unsigned_abs() / 100u64;
        current_value = current_value.rem_euclid(100i64);
    });
    Some(total_count)
}

fn turn_wheel(turn: &str, value: i64, current_value: &i64) -> i64 {
    let moved_value: i64 = match turn {
        "L" => current_value - value,
        "R" => current_value + value,
        _ => panic!("Invalid turn"),
    };
    moved_value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_extra_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(1));
    }
}
