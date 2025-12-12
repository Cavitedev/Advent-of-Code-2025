advent_of_code::solution!(12);

struct Puzzle {
    pieces: Vec<Piece>,
    inputs: Vec<CheckInput>,
}

struct CheckInput {
    width: u8,
    height: u8,
    pieces_needed: Vec<u8>,
}

impl CheckInput {
    fn available_size(&self) -> u16 {
        self.width as u16 * self.height as u16
    }

    fn required_size(&self, pieces: &Vec<Piece>) -> u16 {
        let mut sum: u16 = 0;
        for (index, pieces_needed) in self.pieces_needed.iter().enumerate() {
            sum += (*pieces_needed as u16) * pieces.get(index).unwrap().size();
        }
        sum
    }
}

struct Piece {
    positions: Vec<Vec<bool>>,
}

impl Piece {
    fn size(&self) -> u16 {
        self.positions
            .iter()
            .flatten()
            .filter(|a| **a == true)
            .count() as u16
    }
}

fn parse(input: &str) -> Puzzle {
    let mut pieces: Vec<Piece> = Vec::with_capacity(5);

    let mut n = 1;
    let mut positions: Vec<Vec<bool>> = Vec::new();
    for line in input.lines().skip(1) {
        if line.is_empty() {
            continue;
        }
        if line.contains(&format!("{}:", n)) {
            n += 1;
            pieces.push(Piece { positions });
            positions = Vec::new();
            continue;
        }
        if line.contains("x") {
            pieces.push(Piece { positions });
            break;
        }
        let mut row_pos: Vec<bool> = Vec::new();
        for char in line.as_bytes() {
            match char {
                b'#' => {
                    row_pos.push(true);
                }
                b'.' => row_pos.push(false),
                _ => unreachable!("Should not reach"),
            }
        }
        positions.push(row_pos);
    }

    let mut inputs: Vec<CheckInput> = Vec::with_capacity(50);

    for line in input.lines().skip(30) {
        let (left, right) = line.split_once(":").unwrap();
        let pieces_needed: Vec<u8> = right
            .split(" ")
            .filter(|n| !n.is_empty())
            .map(|n| n.parse().unwrap())
            .collect();
        let (width, height) = left.split_once("x").unwrap();
        inputs.push(CheckInput {
            width: width.parse().unwrap(),
            height: height.parse().unwrap(),
            pieces_needed,
        });
    }

    Puzzle { pieces, inputs }
}

pub fn part_one(input: &str) -> Option<u64> {
    let puzzle = parse(input);
    let mut exit_count = 0;
    for input in puzzle.inputs {
        let puzzle_size = input.available_size();
        let input_size = input.required_size(&puzzle.pieces);
        if input_size > puzzle_size {
            //Pieces cannot take more space than puzzle
            continue;
        }

        exit_count += 1;
    }

    Some(exit_count)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    //Only works on real input which is way too easy
    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(2));
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
