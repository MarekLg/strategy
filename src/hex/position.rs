use std::ops;

use bevy::prelude::Vec2;

use super::direction::Direction;

const SQRT_3: f32 = 1.7320508076;
const SQRT_3_HALF: f32 = 0.8660254038;

pub struct Position {
    pub q: i16,
    pub r: i16,
}

impl Position {
    pub fn new(q: i16, r: i16) -> Self {
        Self { q, r }
    }

    pub fn zero() -> Self {
        Self { q: 0, r: 0 }
    }

    pub fn s(&self) -> i16 {
        -self.q - self.r
    }

    pub fn neighbor(&self, direction: Direction) -> Self {
        self + match direction {
            Direction::NE => Position { q: 1, r: -1 },
            Direction::E => Position { q: 1, r: 0 },
            Direction::SE => Position { q: 0, r: 1 },
            Direction::SW => Position { q: -1, r: 1 },
            Direction::W => Position { q: -1, r: 0 },
            Direction::NW => Position { q: 0, r: -1 },
        }
    }

    pub fn distance(&self, rhs: &Position) -> i16 {
        ((self.q - rhs.q).abs() + (self.q + self.r - rhs.q - rhs.r).abs() + (self.r - rhs.r).abs())
            / 2
    }

    pub fn offset(&self) -> Vec2 {
        Vec2::new(
            self.q as f32 * SQRT_3 + self.r as f32 * SQRT_3_HALF,
            self.r as f32 * -1.5,
        )
    }
}

impl ops::Add<Position> for &Position {
    type Output = Position;

    fn add(self, rhs: Position) -> Self::Output {
        Position::new(self.q + rhs.q, self.r + rhs.r)
    }
}

impl ops::Sub<Position> for &Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        Position::new(self.q - rhs.q, self.r - rhs.r)
    }
}
