use bracket_lib::prelude::Point;

use crate::prelude::*;
use std::cmp;

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl cmp::PartialEq<Position> for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl From<Point> for Position {
    fn from(item: Point) -> Self {
        Position { x:item.x, y:item.y }
    }
}
