advent_of_code::solution!(10);
use std::{
    collections::{HashSet, VecDeque},
};

#[derive(Debug)]
struct Machine {
    light: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

fn parse(input: &str) -> Vec<Machine> {
    let mut machines: Vec<Machine> = Vec::with_capacity(180);

    unsafe {
        const L: usize = 1;
        let mut ptr = input.as_ptr();
        let end_ptr = ptr as usize + input.len() * L;

        ptr = ptr.add(L);

        while (ptr as usize) < end_ptr {
            let mut light: Vec<bool> = Vec::with_capacity(1);

            loop {
                match *ptr {
                    b'.' => light.push(false),
                    b'#' => light.push(true),
                    _ => break,
                }
                ptr = ptr.add(L);
            }
            ptr = ptr.add(L);

            let mut buttons: Vec<Vec<usize>> = Vec::with_capacity(5);

            loop {
                if *ptr == b'{' {
                    break;
                }

                if *ptr == b'(' {
                    ptr = ptr.add(L);
                    let mut button: Vec<usize> = Vec::with_capacity(3);
                    loop {
                        if *ptr == b')' {
                            break;
                        }
                        if *ptr == b',' {
                            ptr = ptr.add(L);
                            continue;
                        }
                        let n = *ptr as usize - b'0' as usize;
                        button.push(n);
                        ptr = ptr.add(L);
                    }

                    buttons.push(button);
                }

                ptr = ptr.add(L);
            }

            let mut joltage: Vec<usize> = Vec::with_capacity(3);
            let mut num: usize = 0;
            ptr = ptr.add(1);
            loop {
                if *ptr == b'}' {
                    joltage.push(num);
                    break;
                }

                if *ptr == b',' {
                    joltage.push(num);
                    num = 0;
                } else {
                    num = num * 10 + *ptr as usize - b'0' as usize;
                }

                ptr = ptr.add(L);
            }
            machines.push(Machine {
                light,
                buttons,
                joltage,
            });
            ptr = ptr.add(3);
        }
    }

    machines
}

fn action_button(state: &Vec<bool>, button: &Vec<usize>) -> Vec<bool> {
    let mut next_state = state.clone();

    for num in button {
        if let Some(v) = next_state.get_mut(*num) {
            *v = !*v;
        }
    }

    next_state
}

fn lowest_combination_num_to_goal(
    start_state: Vec<bool>,
    goal_state: &Vec<bool>,
    buttons: &Vec<Vec<usize>>,
) -> usize {
    if start_state == *goal_state {
        return 0;
    }

    let mut visited: HashSet<Vec<bool>> = HashSet::new();
    let mut queue: VecDeque<(Vec<bool>, usize)> = VecDeque::new();

    visited.insert(start_state.clone());
    queue.push_back((start_state, 0));

    while let Some((state, depth)) = queue.pop_front() {
        for button in buttons {
            let next_state = action_button(&state, button);

            if next_state == *goal_state {
                return depth + 1;
            }

            if visited.insert(next_state.clone()) {
                queue.push_back((next_state, depth + 1));
            }
        }
    }
    0
}

pub fn run_one(input: &str) -> usize {
    let mut count = 0;
    let machines = parse(input);
    for machine in machines {
        count += lowest_combination_num_to_goal(
            vec![false; machine.light.len()],
            &machine.light,
            &machine.buttons,
        );
    }
    count
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(run_one(input))
}

fn action_button_joltage(state: &Vec<usize>, button: &Vec<usize>) -> Vec<usize> {
    let mut next_state = state.clone();

    for num in button {
        if let Some(v) = next_state.get_mut(*num) {
            *v += 1;
        }
    }

    next_state
}

fn lowest_combination_num_to_joltage_goal(
    start_state: Vec<usize>,
    goal_state: &Vec<usize>,
    buttons: &Vec<Vec<usize>>,
) -> usize {
    if start_state == *goal_state {
        return 0;
    }

    let mut visited: HashSet<Vec<usize>> = HashSet::new();
    let mut queue: VecDeque<(Vec<usize>, usize)> = VecDeque::new();

    visited.insert(start_state.clone());
    queue.push_back((start_state, 0));
    while let Some((state, depth)) = queue.pop_front() {
        for button in buttons {
            let next_state = action_button_joltage(&state, button);

            if next_state == *goal_state {
                return depth + 1;
            }
            if next_state
                .iter()
                .zip(goal_state.iter())
                .any(|(x, y)| x > y)
            {
                continue;
            }

            if visited.insert(next_state.clone()) {
                queue.push_back((next_state, depth + 1));
            }
        }
    }
    0
}

pub fn run_two(input: &str) -> usize {
    let mut count = 0;
    let machines = parse(input);
    for (index, machine) in machines.iter().enumerate() {
        dbg!(index);
        count += lowest_combination_num_to_joltage_goal(
            vec![0; machine.light.len()],
            &machine.joltage,
            &machine.buttons,
        );
    }
    count
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(run_two(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }

       #[test]
    fn test_part_two_input() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(33));
    }
}
