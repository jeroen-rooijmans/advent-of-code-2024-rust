// Advent of Code - Day 4: Ceres Search

use std::collections::HashMap;

const INPUT: &str = include_str!("./input.txt");

/// map input into grid, where coord (x,y) points to a char
fn parse_input(input: &str) -> HashMap<(i32, i32), char> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, value)| ((x as i32, y as i32), value))
        })
        .collect::<HashMap<(i32, i32), char>>()
}

fn solve_part_one(input: &str) -> u32 {
    let grid = parse_input(input);

    const DIRECTIONS: [(i32, i32); 8] = [
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, -1),
        (-1, 1),
        (1, 1),
        (-1, -1),
    ];

    // starting at positions that contain 'X', look for 'MAS' in all directions
    let pattern = ['M', 'A', 'S'];
    grid.iter()
        .filter(|(_, value)| **value == 'X')
        .map(|(&idx, _value)| {
            // iterate over directions and check grid for pattern match
            DIRECTIONS
                .iter()
                .map(|&(dx, dy)| {
                    let (mut local_x, mut local_y) = idx;
                    // check if pattern matches for grid positions in given direction
                    pattern.iter().all(|pattern_char| {
                        local_x += dx;
                        local_y += dy;
                        grid.get(&(local_x, local_y)) == Some(pattern_char)
                    })
                })
                // get number of pattern matches
                .filter(|gotcha| *gotcha)
                .count() as u32
        })
        .sum()
}

fn solve_part_two(input: &str) -> u32 {
    let grid = parse_input(input);

    const DIRECTIONS: [(i32, i32); 4] = [(1, -1), (-1, 1), (1, 1), (-1, -1)];

    // starting at positions that contain 'A', and look if position is surrounded by 'M', 'A', 'S'
    let pattern = ['M', 'A', 'S'];
    grid.iter()
        .filter(|(_, value)| **value == 'A')
        // filter out when number of matches is not exactly 2
        .filter(|(&idx, _value)| {
            // iterate over diagonals and check grid for pattern match
            DIRECTIONS
                .iter()
                .map(|&(dx, dy)| {
                    // start by moving two steps in the opposite direction
                    let (mut local_x, mut local_y) = (idx.0 - 2 * dx, idx.1 - 2 * dy);
                    // check if pattern matches for grid positions in given direction
                    pattern.iter().all(|pattern_char| {
                        local_x += dx;
                        local_y += dy;
                        grid.get(&(local_x, local_y)) == Some(pattern_char)
                    })
                })
                .filter(|gotcha| *gotcha)
                .count()
                == 2
        })
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
        let example_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 18);
    }

    #[test]
    fn part2() {
        let example_input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 9);
    }
}
