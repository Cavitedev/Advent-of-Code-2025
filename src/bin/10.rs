advent_of_code::solution!(10);
use std::{
    collections::{HashSet, VecDeque}
};

use z3::{Optimize, SatResult, ast::Int};

#[derive(Debug)]
struct Machine {
    light: Vec<bool>,
    buttons: Vec<Vec<i32>>,
    joltage: Vec<i32>,
}

fn parse(input: &str) -> Vec<Machine> {
    let mut machines: Vec<Machine> = Vec::with_capacity(180);

    unsafe {
        const L: usize = 1;
        let mut ptr = input.as_ptr();
        let end_ptr: usize = ptr as usize + input.len() * L;

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

            let mut buttons: Vec<Vec<i32>> = Vec::with_capacity(5);

            loop {
                if *ptr == b'{' {
                    break;
                }

                if *ptr == b'(' {
                    ptr = ptr.add(L);
                    let mut button: Vec<i32> = Vec::with_capacity(3);
                    loop {
                        if *ptr == b')' {
                            break;
                        }
                        if *ptr == b',' {
                            ptr = ptr.add(L);
                            continue;
                        }
                        let n = *ptr as i32 - b'0' as i32;
                        button.push(n);
                        ptr = ptr.add(L);
                    }

                    buttons.push(button);
                }

                ptr = ptr.add(L);
            }

            let mut joltage: Vec<i32> = Vec::with_capacity(3);
            let mut num: i32 = 0;
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
                    num = num * 10 + *ptr as i32 - b'0' as i32;
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

fn action_button(state: &Vec<bool>, button: &Vec<i32>) -> Vec<bool> {
    let mut next_state = state.clone();

    for num in button {
        if let Some(v) = next_state.get_mut(*num as usize) {
            *v = !*v;
        }
    }

    next_state
}

fn lowest_combination_num_to_goal(
    start_state: Vec<bool>,
    goal_state: &Vec<bool>,
    buttons: &Vec<Vec<i32>>,
) -> i32 {
    if start_state == *goal_state {
        return 0;
    }

    let mut visited: HashSet<Vec<bool>> = HashSet::new();
    let mut queue: VecDeque<(Vec<bool>, i32)> = VecDeque::new();

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

pub fn run_one(input: &str) -> i32 {
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

pub fn part_one(input: &str) -> Option<i32> {
    Some(run_one(input))
}

fn lowest_combination_num_to_joltage_goal(
    goal_state: &Vec<i32>,
    buttons: &Vec<Vec<i32>>,
) -> Option<Vec<u64>> {
    let variables: Vec<Int> = (0..buttons.len())
        .map(|i| Int::fresh_const(&i.to_string()))
        .collect();

    let opt = Optimize::new();

    for variable in &variables {
        opt.assert(&variable.ge(&Int::from_i64(0)));
    }

    for i in 0..goal_state.len() as i32 {
        let indexes_buttons: Vec<usize> = buttons
            .iter()
            .enumerate()
            .filter(|(index, button)| button.contains(&i))
            .map(|b| b.0)
            .collect();

        let affecting_vars: Vec<&Int> = indexes_buttons
            .iter()
            .map(|index| &variables[*index])
            .collect();

        let goal = goal_state[i as usize];

        let sum_expr = Int::add(affecting_vars.as_slice());
        opt.assert(&sum_expr.eq(goal));
    }

    let total_presses = Int::add(&variables.iter().collect::<Vec<_>>());
    opt.minimize(&total_presses);

    match opt.check(&[]) {
        SatResult::Sat => {
            let model = opt.get_model().unwrap();
            let solution: Vec<u64> = variables
                .iter()
                .map(|v| model.eval(v, true).unwrap().as_u64().unwrap())
                .collect();
            Some(solution)
        }
        SatResult::Unsat | SatResult::Unknown => None,
    }
}

pub fn run_two(input: &str) -> u64 {
    let mut count = 0;
    let machines = parse(input);
    for (index, machine) in machines.iter().enumerate() {
        let sol: Vec<u64> =
            lowest_combination_num_to_joltage_goal(&machine.joltage, &machine.buttons).unwrap();
        count += sol.iter().sum::<u64>();
    }
    count
}

pub fn part_two(input: &str) -> Option<u64> {
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
}
