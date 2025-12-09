// Advent of Code - Day 14: Restroom Redoubt
use aoc::coord::Coordinate;
use std::cmp::Ordering;

const INPUT: &str = include_str!("./input.txt");

struct Robot {
    position: Coordinate<isize>,
    dx: isize,
    dy: isize,
}

impl Robot {
    fn position_at(&self, t: usize, room_size: &(usize, usize)) -> Coordinate<isize> {
        let x = (self.position.x + self.dx * t as isize).rem_euclid(room_size.0 as isize);
        let y = (self.position.y + self.dy * t as isize).rem_euclid(room_size.1 as isize);
        Coordinate { x, y }
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .trim()
        .lines()
        .filter_map(|line| {
            let (p_str, v_str) = line.split_once(' ')?;
            let parse_pair = |s: &str| -> Option<(isize, isize)> {
                let s_val = s.strip_prefix("p=").or_else(|| s.strip_prefix("v="))?; // Gets "0,4" or "3,-3"

                let (a_str, b_str) = s_val.split_once(',')?; // Splits into "0" and "4"

                // Fixed: Removed the redundant '::' after parse and correctly handle Option
                let a = a_str.parse().ok()?;
                let b = b_str.parse().ok()?;

                Some((a, b))
            };
            let (px, py) = parse_pair(p_str)?;
            let (vx, vy) = parse_pair(v_str)?;
            Some(Robot {
                position: Coordinate::new(px, py),
                dx: vx,
                dy: vy,
            })
        })
        .collect()
}

fn solve_part_one(input: &str, room_size: &(usize, usize)) -> usize {
    let robots = parse_input(input);
    let mid_x = room_size.0 / 2;
    let mid_y = room_size.1 / 2;

    let (q1, q2, q3, q4) = robots.iter().map(|r| r.position_at(100, room_size)).fold(
        (0, 0, 0, 0),
        |(q1, q2, q3, q4), future_position| {
            match (
                future_position.x.cmp(&(mid_x as isize)),
                future_position.y.cmp(&(mid_y as isize)),
            ) {
                (Ordering::Less, Ordering::Less) => (q1 + 1, q2, q3, q4),
                (Ordering::Greater, Ordering::Less) => (q1, q2 + 1, q3, q4),
                (Ordering::Less, Ordering::Greater) => (q1, q2, q3 + 1, q4),
                (Ordering::Greater, Ordering::Greater) => (q1, q2, q3, q4 + 1),
                _ => (q1, q2, q3, q4), // On the middle lines (ignore)
            }
        },
    );
    q1 * q2 * q3 * q4
}

fn solve_part_two(input: &str) -> usize {
    let robots = parse_input(input);
    let room_size = &(101, 103);
    // because 101 and 103 are prime numbers, the pattern will repeat after 101 * 103 seconds
    // look for lowest variance in robot positions for 101 * 103 timesteps
    let mut best_time = 0;
    let mut lowest_variance = f64::MAX;
    for t in 0..101 * 103 {
        let positions: Vec<Coordinate<isize>> =
            robots.iter().map(|r| r.position_at(t, room_size)).collect();
        let sum_x: isize = positions.iter().map(|p| p.x).sum();
        let sum_y: isize = positions.iter().map(|p| p.y).sum();
        let mean_x = sum_x as f64 / positions.len() as f64;
        let mean_y = sum_y as f64 / positions.len() as f64;
        let var_x: f64 = positions
            .iter()
            .map(|p| (p.x as f64 - mean_x).powi(2))
            .sum();
        let var_y: f64 = positions
            .iter()
            .map(|p| (p.y as f64 - mean_y).powi(2))
            .sum();
        let variance = var_x + var_y;
        if variance < lowest_variance {
            lowest_variance = variance;
            best_time = t as usize;
        }
    }
    best_time
}

fn main() {
    let part_one_answer = solve_part_one(INPUT, &(101, 103));
    println!("Part one:\n{part_one_answer}");
    let part_two_answer = solve_part_two(INPUT);
    println!("Part two:\n{part_two_answer}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let example_input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let answer = crate::solve_part_one(example_input, &(11, 7));
        assert_eq!(answer, 12);
    }
}
