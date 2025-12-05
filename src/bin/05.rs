advent_of_code::solution!(5);
use std::ops::Range;
use std::str::Lines;

#[derive(Debug)]
struct ParsedInput<'a> {
    ranges: Vec<Range<u64>>,
    ingredient_lines: Lines<'a>,
}

fn parse_input(input: &str) -> ParsedInput<'_> {
    unsafe {
        let (first, second) = input.split_once("\n\n").unwrap_unchecked();
        let mut ranges: Vec<Range<u64>> = Vec::with_capacity(180);

        for line_range in first.lines() {
            let (num1, num2) = line_range.split_once("-").unwrap_unchecked();

            ranges.push(Range {
                start: atoi_simd::parse(num1.as_bytes()).unwrap_unchecked(),
                end: atoi_simd::parse(num2.as_bytes()).unwrap_unchecked(),
            });
        }
        ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start));

        let mut merged: Vec<Range<u64>> = Vec::with_capacity(ranges.len());
        let mut iter = ranges.into_iter();
        let mut last = iter.next().unwrap();
        for range in iter {
            if last.end >= range.start - 1 {
                last.end = last.end.max(range.end);
            } else {
                merged.push(last);
                last = range;
            }
        }

        merged.push(last);

        ParsedInput {
            ranges: merged,
            ingredient_lines: second.lines(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut count = 0;
    let parsed_input = parse_input(input);

    unsafe {
        for ingredient_line in parsed_input.ingredient_lines {
            let ingredient: u64 = atoi_simd::parse(ingredient_line.as_bytes()).unwrap_unchecked();
            let search = parsed_input.ranges.binary_search_by(|r| {
                if ingredient < r.start {
                    std::cmp::Ordering::Greater
                } else if ingredient > r.end {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Equal
                }
            });
            count += (search.is_ok()) as u64;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut count = 0;
    let parsed_input = parse_input(input);

    for range in parsed_input.ranges {
        count += range.end - range.start + 1;
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
