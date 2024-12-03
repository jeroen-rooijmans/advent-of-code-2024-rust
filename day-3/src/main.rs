// Advent of Code - Day 3: Mull It Over

const INPUT: &str = include_str!("./input.txt");

fn solve_part_one(input: &str) -> u32 {
    let mut mul_instructions: Vec<(u32, u32)> = vec![];
    let mut chars = input.chars().peekable();
    while let Some(char) = chars.next() {
        if char == 'm'
            && chars.next() == Some('u')
            && chars.next() == Some('l')
            && chars.next() == Some('(')
        {
            // grab first number
            let mut left_number = String::new();
            while let Some(&char) = chars.peek() {
                if char.is_ascii_digit() {
                    left_number.push(char);
                    chars.next();
                } else {
                    break;
                }
            }

            // check for `,`
            if chars.peek() != Some(&',') {
                continue;
            } else {
                chars.next();
            }
            // grab second number
            let mut right_number = String::new();
            while let Some(&char) = chars.peek() {
                if char.is_ascii_digit() {
                    right_number.push(char);
                    chars.next();
                } else {
                    break;
                }
            }

            // check for `)`
            if chars.peek() != Some(&')') {
                continue;
            } else {
                chars.next();
            }

            // parse numbers
            if let (Some(l), Some(r)) = (left_number.parse().ok(), right_number.parse().ok()) {
                mul_instructions.push((l, r));
            } else {
                continue;
            }
        }
    }
    mul_instructions.iter().map(|(x, y)| x * y).sum()
}

fn solve_part_two(input: &str) -> u32 {
    let mut enable_mul = true;
    let mut mul_instructions: Vec<(u32, u32)> = vec![];
    let mut chars = input.chars().peekable();
    while let Some(char) = chars.next() {
        // parse and handle mul instruction
        if char == 'm'
            && chars.next() == Some('u')
            && chars.next() == Some('l')
            && chars.next() == Some('(')
        {
            // grab first number
            let mut left_number = String::new();
            while let Some(&char) = chars.peek() {
                if char.is_ascii_digit() {
                    left_number.push(char);
                    chars.next();
                } else {
                    break;
                }
            }

            // check for `,`
            if chars.peek() != Some(&',') {
                continue;
            } else {
                chars.next();
            }
            // grab second number
            let mut right_number = String::new();
            while let Some(&char) = chars.peek() {
                if char.is_ascii_digit() {
                    right_number.push(char);
                    chars.next();
                } else {
                    break;
                }
            }

            // check for `)`
            if chars.peek() != Some(&')') {
                continue;
            } else {
                chars.next();
            }

            // parse numbers
            if let (Some(l), Some(r)) = (left_number.parse().ok(), right_number.parse().ok()) {
                if enable_mul {
                    mul_instructions.push((l, r));
                }
            } else {
                continue;
            }
        // parse and handle do and don't instructions
        } else if char == 'd' && chars.next() == Some('o') {
            let char = chars.next();
            if char == Some('(') && chars.next() == Some(')') {
                enable_mul = true;
            } else if char == Some('n')
                && chars.next() == Some('\'')
                && chars.next() == Some('t')
                && chars.next() == Some('(')
                && chars.next() == Some(')')
            {
                enable_mul = false;
            }
        }
    }
    mul_instructions.iter().map(|(x, y)| x * y).sum()
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
        let example_input =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 161);
    }

    #[test]
    fn part2() {
        let example_input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 48);
    }
}
