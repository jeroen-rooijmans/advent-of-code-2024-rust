// Advent of Code - Day 8: Resonant Collinearity
use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use aoc::Coordinate;

const INPUT: &str = include_str!("./input.txt");

fn antinodes(
    coord1: Coordinate<usize>,
    coord2: Coordinate<usize>,
    max_height: usize,
    max_width: usize,
) -> (Option<Coordinate<usize>>, Option<Coordinate<usize>>) {
    let antenna1 = (coord1.x as i32, coord1.y as i32);
    let antenna2 = (coord2.x as i32, coord2.y as i32);

    // vector from antenna1 to antenna2
    let (dx, dy) = (antenna2.0 - antenna1.0, antenna2.1 - antenna1.1);

    let antinode1 = (antenna1.0 - dx, antenna1.1 - dy);
    let antinode2 = (antenna2.0 + dx, antenna2.1 + dy);

    // convert to coordinates if antinode is in bounds
    let try_into_coord = |p: (i32, i32)| {
        if p.0 >= 0 && p.1 >= 0 && p.0 < max_width as i32 && p.1 < max_height as i32 {
            Some(Coordinate {
                x: p.0 as usize,
                y: p.1 as usize,
            })
        } else {
            None
        }
    };

    (try_into_coord(antinode1), try_into_coord(antinode2))
}

fn resonant_antinodes(
    coord1: Coordinate<usize>,
    coord2: Coordinate<usize>,
    max_height: usize,
    max_width: usize,
) -> Vec<Coordinate<usize>> {
    let antenna1 = (coord1.x as i32, coord1.y as i32);
    let antenna2 = (coord2.x as i32, coord2.y as i32);

    // vector from antenna1 to antenna2
    let (dx, dy) = (antenna2.0 - antenna1.0, antenna2.1 - antenna1.1);

    let mut points = Vec::new();

    // Traverse line in the negative direction
    let mut x = antenna1.0;
    let mut y = antenna1.1;
    while x >= 0 && y >= 0 && x < max_width as i32 && y < max_height as i32 {
        points.push(Coordinate {
            x: x as usize,
            y: y as usize,
        });
        x -= dx;
        y -= dy;
    }

    // Reset to the starting point and traverse in the positive direction
    x = antenna1.0;
    y = antenna1.1;
    while x >= 0 && y >= 0 && x < max_width as i32 && y < max_height as i32 {
        points.push(Coordinate {
            x: x as usize,
            y: y as usize,
        });
        x += dx;
        y += dy;
    }
    points
}

fn solve_part_one(input: &str) -> usize {
    let max_height = input.lines().count();
    let max_width = input.lines().next().map_or(0, |line| line.len());

    let antenna_freqs: HashSet<char> = input.chars().filter(|&c| c != '.' && c != '\n').collect();

    let antenna_coords: HashMap<char, Vec<Coordinate<usize>>> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let freq_ref = &antenna_freqs;
            line.chars().enumerate().filter_map(move |(x, ch)| {
                freq_ref.contains(&ch).then_some((ch, Coordinate { x, y }))
            })
        })
        .fold(HashMap::new(), |mut acc, (ch, coord)| {
            acc.entry(ch).or_default().push(coord);
            acc
        });

    antenna_coords
        .iter()
        .flat_map(|(_freq, coords)| {
            coords.iter().combinations(2).flat_map(|pair| {
                let (antinode1, antinode2) = antinodes(*pair[0], *pair[1], max_height, max_width);
                [antinode1, antinode2].into_iter().flatten()
            })
        })
        .unique()
        .count()
}

fn solve_part_two(input: &str) -> usize {
    let max_height = input.lines().count();
    let max_width = input.lines().next().map_or(0, |line| line.len());

    let antenna_freqs: HashSet<char> = input.chars().filter(|&c| c != '.' && c != '\n').collect();

    let antenna_coords: HashMap<char, Vec<Coordinate<usize>>> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let freq_ref = &antenna_freqs;
            line.chars().enumerate().filter_map(move |(x, ch)| {
                freq_ref.contains(&ch).then_some((ch, Coordinate { x, y }))
            })
        })
        .fold(HashMap::new(), |mut acc, (ch, coord)| {
            acc.entry(ch).or_default().push(coord);
            acc
        });

    antenna_coords
        .iter()
        .flat_map(|(_freq, coords)| {
            coords
                .iter()
                .combinations(2)
                .flat_map(|pair| resonant_antinodes(*pair[0], *pair[1], max_height, max_width))
        })
        .unique()
        .count()
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
        let example_input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 14);
    }

    #[test]
    fn part2() {
        let example_input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 34);
    }
}
