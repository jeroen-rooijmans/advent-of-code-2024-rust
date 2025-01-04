// Advent of Code - Day 6: Guard Gallivant

use std::collections::HashSet;

use aoc::coord::Coordinate;

const INPUT: &str = include_str!("./input.txt");

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        };
    }
}

#[derive(Debug)]
struct Grid {
    height: usize,
    width: usize,
    obstacles: HashSet<Coordinate<usize>>,
}

fn parse_input(input: &str) -> (Grid, Coordinate<usize>) {
    let lines = input.lines().collect::<Vec<_>>();
    let height = lines.len();
    let width = lines.first().map_or(0, |line| line.len());
    let mut guard_position = Coordinate {
        x: usize::MAX,
        y: usize::MAX,
    };
    let obstacles = lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, c)| match c {
                    '^' => {
                        guard_position = Coordinate { x, y };
                        None
                    }
                    '#' => Some(Coordinate { x, y }),
                    _ => None,
                })
                .collect::<HashSet<Coordinate<_>>>()
        })
        .collect();
    (
        Grid {
            height,
            width,
            obstacles,
        },
        guard_position,
    )
}

fn patrol(
    grid: &Grid,
    start_position: &Coordinate<usize>,
    start_direction: &Direction,
) -> HashSet<Coordinate<usize>> {
    let mut guard_position = *start_position;
    let mut guard_direction = *start_direction;
    let mut visited_positions = HashSet::new();
    visited_positions.insert(guard_position);
    loop {
        let next_position = match &guard_direction {
            Direction::Up => {
                if guard_position.y == 0 {
                    break;
                }
                guard_position - Coordinate { x: 0, y: 1 }
            }
            Direction::Right => {
                if guard_position.x + 1 >= grid.width {
                    break;
                }
                guard_position + Coordinate { x: 1, y: 0 }
            }
            Direction::Down => {
                if guard_position.y + 1 >= grid.height {
                    break;
                }
                guard_position + Coordinate { x: 0, y: 1 }
            }
            Direction::Left => {
                if guard_position.x == 0 {
                    break;
                }
                guard_position - Coordinate { x: 1, y: 0 }
            }
        };
        if grid.obstacles.contains(&next_position) {
            guard_direction.turn();
        } else {
            visited_positions.insert(next_position);
            guard_position = next_position;
        }
    }
    visited_positions
}

fn solve_part_one(input: &str) -> u32 {
    let (grid, guard_position) = parse_input(input);
    let visited_positions = patrol(&grid, &guard_position, &Direction::Up);
    visited_positions.len() as u32
}

fn solve_part_two(input: &str) -> u32 {
    let guard_direction = Direction::Up;
    let (grid, guard_position) = parse_input(input);
    let mut visited_positions = patrol(&grid, &guard_position, &guard_direction);
    visited_positions.remove(&guard_position);
    let new_obstacle_count = visited_positions
        .iter()
        .filter(|new_obstacle| {
            let mut loop_position = guard_position;
            let mut loop_direction = guard_direction;
            let mut visisted_posdirs: HashSet<(Coordinate<usize>, Direction)> =
                HashSet::from([(loop_position, loop_direction)]);
            loop {
                let next_position = match &loop_direction {
                    Direction::Up => {
                        if loop_position.y == 0 {
                            break false;
                        }
                        loop_position - Coordinate { x: 0, y: 1 }
                    }
                    Direction::Right => {
                        if loop_position.x + 1 >= grid.width {
                            break false;
                        }
                        loop_position + Coordinate { x: 1, y: 0 }
                    }
                    Direction::Down => {
                        if loop_position.y + 1 >= grid.height {
                            break false;
                        }
                        loop_position + Coordinate { x: 0, y: 1 }
                    }
                    Direction::Left => {
                        if loop_position.x == 0 {
                            break false;
                        }
                        loop_position - Coordinate { x: 1, y: 0 }
                    }
                };
                if grid.obstacles.contains(&next_position) || next_position == **new_obstacle {
                    loop_direction.turn();
                } else if visisted_posdirs.contains(&(next_position, loop_direction)) {
                    break true;
                } else {
                    visisted_posdirs.insert((next_position, loop_direction));
                    loop_position = next_position;
                }
            }
        })
        .count();
    new_obstacle_count as u32
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
        let example_input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let answer = crate::solve_part_one(example_input);
        assert_eq!(answer, 41);
    }

    #[test]
    fn part2() {
        let example_input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let answer = crate::solve_part_two(example_input);
        assert_eq!(answer, 6);
    }
}
