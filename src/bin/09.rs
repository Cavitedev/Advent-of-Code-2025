advent_of_code::solution!(9);

struct Point {
    x: i64,
    y: i64,
}

fn parse_points_input(input: &str) -> Vec<Point> {
    let mut points: Vec<Point> = Vec::with_capacity(500);

    for line in input.lines() {
        let positions: Vec<i64> = line
            .split(",")
            .map(|values| values.parse::<i64>().unwrap())
            .collect();
        points.push(Point {
            x: positions[0],
            y: positions[1],
        });
    }

    points
}

pub fn run_one(input: &str) -> i64 {
    let points = parse_points_input(input);

    let mut best_area = 0;
    for i in 0..points.len() {
        for j in 1..points.len() {
            let start = &points[i];
            let end = &points[j];
            let area = ((start.x - end.x).abs() + 1) * ((start.y - end.y).abs() + 1);
            if area > best_area {
                best_area = area;
            }
        }
    }

    best_area
}

pub fn part_one(input: &str) -> Option<i64> {
    Some(run_one(input))
}

struct HorizontalLine {
    y: i64,
    start_x: i64,
    end_x: i64,
}

struct VerticalLine {
    x: i64,
    start_y: i64,
    end_y: i64,
}

fn add_line_from_points(
    horizontal_lines: &mut Vec<HorizontalLine>,
    vertical_lines: &mut Vec<VerticalLine>,
    point: &Point,
    point2: &Point,
) {
    if point.y == point2.y {
        let start = point.x.min(point2.x);
        let end = point.x.max(point2.x);
        horizontal_lines.push(HorizontalLine {
            y: point.y,
            start_x: start,
            end_x: end,
        });
    } else if point.x == point2.x {
        let start = point.y.min(point2.y);
        let end = point.y.max(point2.y);
        vertical_lines.push(VerticalLine {
            x: point.x,
            start_y: start,
            end_y: end,
        });
    }
}

fn lines_from_points(points: &Vec<Point>) -> (Vec<HorizontalLine>, Vec<VerticalLine>) {
    let mut horizontal_lines: Vec<HorizontalLine> = Vec::with_capacity(250);
    let mut vertical_lines: Vec<VerticalLine> = Vec::with_capacity(250);

    for i in 0..points.len() - 1 {
        let point = &points[i];
        let point2 = &points[i + 1];

        add_line_from_points(&mut horizontal_lines, &mut vertical_lines, point, point2);
    }

    add_line_from_points(
        &mut horizontal_lines,
        &mut vertical_lines,
        points.last().unwrap(),
        &points[0],
    );

    (horizontal_lines, vertical_lines)
}

fn is_vertical_line_between_points(
    vertical_lines: &Vec<VerticalLine>,
    start: &Point,
    end: &Point,
) -> bool {
    for line in vertical_lines {
        if line.end_y >= start.y
            && line.start_y <= start.y
            && line.x > start.x.min(end.x)
            && line.x < start.x.max(end.x)
        {
            return true;
        }
    }

    false
}

fn is_horizontal_line_between_points(
    horizontal_lines: &Vec<HorizontalLine>,
    start: &Point,
    end: &Point,
) -> bool {
    for line in horizontal_lines {
        if line.end_x >= start.x
            && line.start_x <= start.x
            && line.y > start.y.min(end.y)
            && line.y < start.y.max(end.y)
        {
            return true;
        }
    }

    false
}

fn is_valid_area(
    horizontal_lines: &Vec<HorizontalLine>,
    vertical_lines: &Vec<VerticalLine>,
    start: &Point,
    end: &Point,
) -> bool {
    let check1 = !is_vertical_line_between_points(
        vertical_lines,
        start,
        &Point {
            y: start.y,
            x: end.x,
        },
    );

    let check2 = !is_horizontal_line_between_points(
        horizontal_lines,
        &Point {
            y: start.y,
            x: end.x,
        },
        end,
    );

    let check3 = !is_vertical_line_between_points(
        vertical_lines,
        end,
        &Point {
            y: end.y,
            x: start.x,
        },
    );

    let check4 = !is_horizontal_line_between_points(
        horizontal_lines,
        &Point {
            y: end.y,
            x: start.x,
        },
        start,
    );

    check1 && check2 && check3 && check4
}

pub fn run_two(input: &str) -> i64 {
    let points = parse_points_input(input);
    let (horizontal_lines, vertical_lines) = lines_from_points(&points);
    let mut best_area = 0;
    for i in 0..points.len() {
        for j in 1..points.len() {
            let start = &points[i];
            let end = &points[j];
            let area = ((start.x - end.x).abs() + 1) * ((start.y - end.y).abs() + 1);
            if area > best_area {
                if is_valid_area(&horizontal_lines, &vertical_lines, start, end) {
                    best_area = area;
                }
            }
        }
    }

    best_area
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(run_two(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}
