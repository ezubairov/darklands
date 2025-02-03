use crate::prelude::*;
use std::cmp;

#[derive(Component, Copy, Clone, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn new_from2d(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl cmp::PartialEq<Position> for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
