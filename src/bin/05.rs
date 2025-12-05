advent_of_code::solution!(5);

#[derive(Debug)]
struct Range {
    min: u64,
    max: u64,
}

#[derive(Debug)]
struct ParsedInput {
    ranges: Vec<Range>,
    ingredients: Vec<u64>,
}

fn parse_input(input: &str) -> ParsedInput {
    let (first, second) = input.split_once("\n\n").unwrap();
    let mut ranges: Vec<Range> = Vec::with_capacity(50);

    for line_range in first.lines() {
        let (num1, num2) = line_range.split_once("-").unwrap();

        ranges.push(Range {
            min: num1.parse().unwrap(),
            max: num2.parse().unwrap(),
        });
    }

    ranges.sort_by(|a, b| a.min.cmp(&b.min));

    let mut merged: Vec<Range> = Vec::with_capacity(ranges.len());

    for range in ranges {
        if let Some(last) = merged.last_mut() {
            if last.max >= range.min - 1 {
                last.max = last.max.max(range.max);
            } else {
                merged.push(range);
            }
        } else {
            merged.push(range);
        }
    }

    let mut ingredients: Vec<u64> = Vec::with_capacity(100);
    for ingredient in second.lines() {
        ingredients.push(ingredient.parse().unwrap());
    }

    ParsedInput {
        ranges: merged,
        ingredients: ingredients,
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut count = 0;
    let parsed_input = parse_input(input);

    for ingredient in parsed_input.ingredients {
        let search = parsed_input.ranges.binary_search_by(|r| {
            if ingredient < r.min {
                std::cmp::Ordering::Greater
            } else if ingredient > r.max {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });
        count += (search.is_ok()) as u64;
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut count = 0;
    let parsed_input = parse_input(input);

    for range in parsed_input.ranges {
        count += range.max - range.min + 1;
    }

    Some(count)
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
        assert_eq!(result, Some(14));
    }
}
