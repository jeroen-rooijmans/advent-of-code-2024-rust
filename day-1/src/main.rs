// Advent of Code - Day 1: Historian Hysteria

use std::iter::zip;

const INPUT: &str = include_str!("./input.txt");

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<u32>().unwrap(),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .unzip()
}

fn solve_part_one(input: &str) -> u32 {
    let (mut left, mut right) = parse_input(input);

    // sort lists
    left.sort();
    right.sort();

    // compute sum of distances
    zip(left, right).map(|(l, r)| l.abs_diff(r)).sum()
}

fn solve_part_two(input: &str) -> u32 {
    let (left, right) = parse_input(input);

    // compute similarity score by multiplying each element in left list with its count in right list
    left.iter()
        .map(|l| {
            let count = right.iter().filter(|r| *r == l).count() as u32;
            l * count
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
        let example_input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 11);
    }

    #[test]
    fn part2() {
        let example_input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 31);
    }
}
