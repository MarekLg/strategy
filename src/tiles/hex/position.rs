use std::ops;

use bevy::prelude::Vec2;

use super::{
    corner::Corner,
    edge::{Edge, EDGES},
};

const WIDTH: f32 = 1.7320508076;
const WIDTH_1_2: f32 = 0.8660254038;
#[allow(dead_code)]
const HEIGHT: f32 = 2.0;
const HEIGHT_1_2: f32 = 1.0;
const HEIGHT_3_4: f32 = 1.5;
const HEIGHT_1_4: f32 = 0.5;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub q: i16,
    pub r: i16,
}

impl Position {
    pub const ZERO: Position = Self { q: 0, r: 0 };

    pub fn new(q: i16, r: i16) -> Self {
        Self { q, r }
    }

    pub fn s(&self) -> i16 {
        -self.q - self.r
    }

    pub fn neighbor(&self, edge: &Edge) -> Self {
        self + match edge {
            Edge::NE => Position { q: 1, r: -1 },
            Edge::E => Position { q: 1, r: 0 },
            Edge::SE => Position { q: 0, r: 1 },
            Edge::SW => Position { q: -1, r: 1 },
            Edge::W => Position { q: -1, r: 0 },
            Edge::NW => Position { q: 0, r: -1 },
        }
    }

    pub fn neighbors(&self) -> [Self; 6] {
        EDGES.map(|edge| self.neighbor(&edge))
    }

    pub fn distance(&self, rhs: &Position) -> i16 {
        ((self.q - rhs.q).abs() + (self.q + self.r - rhs.q - rhs.r).abs() + (self.r - rhs.r).abs())
            / 2
    }

    pub fn offset(&self) -> Vec2 {
        Vec2::new(
            self.q as f32 * WIDTH + self.r as f32 * WIDTH_1_2,
            self.r as f32 * -HEIGHT_3_4,
        )
    }

    pub fn offset_corner(&self, corner: &Corner) -> Vec2 {
        self.offset()
            + match corner {
                Corner::N => Vec2 {
                    x: 0.0,
                    y: HEIGHT_1_2,
                },
                Corner::NE => Vec2 {
                    x: WIDTH_1_2,
                    y: HEIGHT_1_4,
                },
                Corner::SE => Vec2 {
                    x: WIDTH_1_2,
                    y: -HEIGHT_1_4,
                },
                Corner::S => Vec2 {
                    x: 0.0,
                    y: -HEIGHT_1_2,
                },
                Corner::SW => Vec2 {
                    x: -WIDTH_1_2,
                    y: -HEIGHT_1_4,
                },
                Corner::NW => Vec2 {
                    x: -WIDTH_1_2,
                    y: HEIGHT_1_4,
                },
            }
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

impl ops::Mul<u16> for &Position {
    type Output = Position;

    fn mul(self, rhs: u16) -> Self::Output {
        Position::new(self.q * rhs as i16, self.r * rhs as i16)
    }
}
