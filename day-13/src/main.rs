// Advent of Code - Day 13: Claw Contraption
use std::str::FromStr;

const INPUT: &str = include_str!("./input.txt");

#[derive(Debug, Clone, Copy)]
struct Button {
    dx: isize,
    dy: isize,
    cost: isize,
}

#[derive(Debug)]
struct Machine {
    a: Button,
    b: Button,
    prize_x: isize,
    prize_y: isize,
}

impl Machine {
    fn solve(&self) -> Option<isize> {
        let determinant = self.a.dx * self.b.dy - self.b.dx * self.a.dy;
        if determinant == 0 {
            // no solution
            return None;
        }

        // find numerators (Da and Db)
        let da = self.prize_x * self.b.dy - self.b.dx * self.prize_y;
        let db = self.a.dx * self.prize_y - self.prize_x * self.a.dy;

        // check for integer solutions
        if da % determinant != 0 || db % determinant != 0 {
            return None;
        }

        let a = da / determinant;
        let b = db / determinant;

        // check for non-negative solutions
        if a >= 0 && b >= 0 {
            Some(a * self.a.cost + b * self.b.cost)
        } else {
            None
        }
    }
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .split("\n\n")
        .filter_map(|block| {
            let numeric_string = block.replace(|c: char| !c.is_ascii_digit() && c != '-', " ");

            let numbers: Vec<isize> = numeric_string
                .split_whitespace()
                .filter_map(|s| isize::from_str(s).ok())
                .collect();

            if numbers.len() != 6 {
                return None;
            }

            Some(Machine {
                a: Button {
                    dx: numbers[0],
                    dy: numbers[1],
                    cost: 3,
                },
                b: Button {
                    dx: numbers[2],
                    dy: numbers[3],
                    cost: 1,
                },
                prize_x: numbers[4],
                prize_y: numbers[5],
            })
        })
        .collect()
}

fn solve_part_one(input: &str) -> isize {
    let machines = parse_input(input);
    machines.iter().filter_map(Machine::solve).sum()
}

fn solve_part_two(input: &str) -> isize {
    let machines = parse_input(input);
    // fix conversion error
    let offset = 10_000_000_000_000;
    let machines: Vec<Machine> = machines
        .iter()
        .map(|m| Machine {
            a: m.a,
            b: m.b,
            prize_x: m.prize_x + offset,
            prize_y: m.prize_y + offset,
        })
        .collect();
    machines.iter().filter_map(Machine::solve).sum()
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
        let example_input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 480);
    }

    #[test]
    fn part2() {
        let example_input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 875318608908);
    }
}
