use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct Coordinate<T> {
    pub x: T,
    pub y: T,
}

impl<T: Ord> Ord for Coordinate<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.x.cmp(&other.x).then(self.y.cmp(&other.y))
    }
}

impl<T: Ord> PartialOrd for Coordinate<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Add<Output = T>> Add for Coordinate<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign for Coordinate<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<Output = T>> Sub for Coordinate<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: SubAssign> SubAssign for Coordinate<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Coordinate<usize> {
    pub fn adjacent(self) -> [Option<Self>; 4] {
        let up = (self.y > 0).then(|| self - Self { x: 0, y: 1 });
        let right = (self.x < usize::MAX).then(|| self + Self { x: 1, y: 0 });
        let down = (self.y < usize::MAX).then(|| self + Self { x: 0, y: 1 });
        let left = (self.x > 0).then(|| self - Self { x: 1, y: 0 });
        [up, right, down, left]
    }

    pub fn surrounding(self) -> [Option<Self>; 8] {
        let up = (self.y > 0).then(|| self - Self { x: 0, y: 1 });
        let topright = (self.x < usize::MAX && self.y > 0)
            .then(|| self + Self { x: 1, y: 0 } - Self { x: 0, y: 1 });
        let right = (self.x < usize::MAX).then(|| self + Self { x: 1, y: 0 });
        let bottomright =
            (self.x < usize::MAX && self.y < usize::MAX).then(|| self + Self { x: 1, y: 1 });
        let down = (self.y < usize::MAX).then(|| self + Self { x: 0, y: 1 });
        let bottomleft = (self.x > 0 && self.y < usize::MAX)
            .then(|| self - Self { x: 1, y: 0 } + Self { x: 0, y: 1 });
        let left = (self.x > 0).then(|| self - Self { x: 1, y: 0 });
        let topleft = (self.x > 0 && self.y > 0).then(|| self - Self { x: 1, y: 1 });
        [
            up,
            topright,
            right,
            bottomright,
            down,
            bottomleft,
            left,
            topleft,
        ]
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
