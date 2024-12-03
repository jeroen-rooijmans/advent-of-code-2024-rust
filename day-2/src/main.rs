// Advent of Code - Day 2: Red-Nosed Reports

const INPUT: &str = include_str!("./input.txt");

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>()
}

/// Safety check for report
fn safety_check(report: &[u32]) -> bool {
    // compute differences between adjecent levels for report
    let diff = report
        .windows(2)
        .map(|w| w[0] as i32 - w[1] as i32)
        .collect::<Vec<i32>>();
    // report is safe when the levels are either all increasing or all decreasing, and
    // any two adjacent levels differ by at least one and at most three.
    (diff.iter().all(|d| *d < 0) || diff.iter().all(|d| *d > 0))
        && diff.iter().all(|d| d.abs() <= 3)
}

/// Safety check with Problem Dampner for report
fn safety_check_dampner(report: &[u32]) -> bool {
    // initial safety check
    if safety_check(report) {
        return true;
    } else {
        // repeatedly modify report naively and check if the result is safe
        for i in 0..report.len() {
            let mut dampner_report = report.to_vec();
            dampner_report.remove(i);
            if safety_check(&dampner_report) {
                return true;
            }
        }
    }
    // if none of the permutations pass the safety check, return false
    false
}

fn solve_part_one(input: &str) -> u32 {
    let reports = parse_input(input);
    // count the number of safe reports
    reports.iter().filter(|report| safety_check(report)).count() as u32
}

fn solve_part_two(input: &str) -> u32 {
    let reports = parse_input(input);
    // check if all levels are safe given the Problem Dampner for each report
    reports
        .iter()
        .filter(|report| safety_check_dampner(report))
        .count() as u32
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
        let example_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 2);
    }

    #[test]
    fn part2() {
        let example_input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 4);
    }
}
