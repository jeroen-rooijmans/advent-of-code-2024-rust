// Advent of Code - Day 7: Bridge Repair

use std::iter::repeat;

use itertools::Itertools;

const INPUT: &str = include_str!("./input.txt");

fn parse_into_equation(line: &str) -> (u64, Vec<u64>) {
    let (test_value, numbers) = line.split_once(":").unwrap();
    let test_value = test_value.parse().unwrap();
    let numbers = numbers
        .split_whitespace()
        .map(|num| num.parse().unwrap())
        .collect();
    (test_value, numbers)
}

fn operator_combinations(operators: &[char], n: usize) -> Vec<Vec<char>> {
    repeat(operators)
        .take(n)
        .multi_cartesian_product()
        .map(|v| v.into_iter().cloned().collect())
        .collect()
}

fn evaluate_equation(test_value: &u64, numbers: &[u64], operators: &[char]) -> bool {
    let mut result: u64 = numbers[0];
    for (i, &op) in operators.iter().enumerate() {
        match op {
            '*' => result *= numbers[i + 1],
            '+' => result += numbers[i + 1],
            '|' => {
                let factor = 10_u64.pow(numbers[i + 1].ilog10() + 1);
                result = result * factor + numbers[i + 1]
            }
            _ => panic!("Unknown operator!"),
        }
    }
    result == *test_value
}

fn solve_part_one(input: &str) -> u64 {
    let equations = input.lines().map(parse_into_equation).collect::<Vec<_>>();
    equations
        .iter()
        .filter_map(|(test_value, numbers)| {
            operator_combinations(&['*', '+'], numbers.len() - 1)
                .iter()
                .any(|ops| evaluate_equation(test_value, numbers, ops))
                .then_some(*test_value)
        })
        .sum()
}

fn solve_part_two(input: &str) -> u64 {
    let equations = input.lines().map(parse_into_equation).collect::<Vec<_>>();
    equations
        .iter()
        .filter_map(|(test_value, numbers)| {
            operator_combinations(&['*', '+', '|'], numbers.len() - 1)
                .iter()
                .any(|ops| evaluate_equation(test_value, numbers, ops))
                .then_some(*test_value)
        })
        .sum()
}

fn main() {
    let part_one_answer = solve_part_one(INPUT);
    println!("Part one:\n{part_one_answer}");
    let part_two_answer = solve_part_two(INPUT);
    println!("Part two:\n{part_two_answer}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 3749);
    }

    #[test]
    fn part2() {
        let example_input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 11387);
    }
}
