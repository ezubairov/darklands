use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileType {
    Wall,
    Floor,
    Void,
}

const PASSABLE_TILE_TYPES: [TileType; 1] = [TileType::Floor];

#[derive(Debug)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub occupation: Vec<Option<Entity>>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Void; NUM_TILES],
            occupation: vec![None; NUM_TILES],
        }
    }

    pub fn in_bounds<T: Into<Position>>(&self, position: T) -> bool {
        let position = position.into();
        position.x >= 0
            && position.x < SCREEN_WIDTH
            && position.y >= 0
            && position.y < SCREEN_HEIGHT
    }

    pub fn tile_passable<T: Into<Position>>(&self, position: T) -> bool {
        let position = position.into();
        self.in_bounds(position)
            && (PASSABLE_TILE_TYPES.contains(&self.tiles[map_idx(position.x, position.y)]))
    }

    pub fn try_idx(&self, position: Position) -> Option<usize> {
        if !self.in_bounds(position) {
            None
        } else {
            Some(map_idx(position.x, position.y))
        }
    }
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
