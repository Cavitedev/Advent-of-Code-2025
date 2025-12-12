#![feature(portable_simd)]

advent_of_code::solution!(12);

use core::simd::Simd;
use std::simd::num::SimdUint;

pub fn run(input: &str) -> u16 {
    unsafe {
        let ptr_start = input.as_ptr();
        let end_ptr: usize = ptr_start as usize + input.len();

        let mut count_sizes: [u16; 6] = [0; 6];

        for i in 0..96 {
            if *ptr_start.add(i) == b'#' {
                let idx: usize = i / 16;
                count_sizes[idx] += 1;
            }
        }

        let mut ptr = ptr_start.add(96);
        let mut count = 0;

        while (ptr as usize) < end_ptr {
            let (p, w) = parse_until(ptr, b'x');
            let (mut p, h) = parse_until(p, b':');

            p = p.add(1);
            let (p, n1) = parse_until(p, b' ');
            let (p, n2) = parse_until(p, b' ');
            let (p, n3) = parse_until(p, b' ');
            let (p, n4) = parse_until(p, b' ');
            let (p, n5) = parse_until(p, b' ');
            let (p, n6) = parse_until(p, b'\n');
            let values: [u16; 6] = [n1, n2, n3, n4, n5, n6];

            let va = Simd::<u16, 6>::from_array(count_sizes);
            let vb = Simd::<u16, 6>::from_array(values);

            let vc = va * vb;
            let input_size = vc.reduce_sum();

            if w * h > input_size {
                count += 1;
            }

            ptr = p;
        }

        count
    }
}

fn parse_until(mut ptr: *const u8, delimiter: u8) -> (*const u8, u16) {
    unsafe {
        let mut n = *ptr as u16 - b'0' as u16;
        ptr = ptr.add(1);

        loop {
            let value = *ptr;
            if value == delimiter {
                break;
            }

            n = n * 10 + value as u16 - b'0' as u16;
            ptr = ptr.add(1);
        }
        ptr = ptr.add(1);
        (ptr, n)
    }
}

pub fn part_one(input: &str) -> Option<u16> {
    Some(run(input))
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(536));
    }
}
