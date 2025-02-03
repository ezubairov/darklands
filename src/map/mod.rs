use crate::prelude::*;
use bracket_bevy::prelude::*;
use bracket_lib::prelude::Rect;
use bracket_lib::terminal::RGB;

mod components;
use components::*;

const NUM_ROOMS: usize = 5;

#[derive(Resource)]
pub struct MapBuilder {
    pub map: Map,
    walls: Vec<Rect>,
    rooms: Vec<Rect>,
}
impl MapBuilder {
    fn new() -> Self {
        let mut mb = Self {
            map: Map::new(),
            walls: Vec::new(),
            rooms: Vec::new(),
        };

        mb.fill(TileType::Void);
        mb.build_random_rooms();
        mb.build_corridors();

        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self) {
        let mut rng = rand::rng();

        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.random_range(2..SCREEN_WIDTH - 12),
                rng.random_range(2..SCREEN_HEIGHT - 12),
                rng.random_range(2..12),
                rng.random_range(2..12),
            );

            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }
            if !overlap {
                let wall = Rect::with_exact(room.x1 - 1, room.y1 - 1, room.x2 + 1, room.y2 + 1);
                // First make the floor space that will be the room
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                // now place the walls around it
                wall.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        if self.map.tiles[idx] == TileType::Void {
                            self.map.tiles[idx] = TileType::Wall;
                        }
                    }
                });
                self.rooms.push(room);
                self.walls.push(wall);
            }
        }
    }

    fn apply_horizontal_tunnel_walls(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Position { x, y }) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
            if let Some(idx) = self.map.try_idx(Position { x, y: y - 1 }) {
                if self.map.tiles[idx as usize] == TileType::Void {
                    self.map.tiles[idx as usize] = TileType::Wall;
                }
            }
            if let Some(idx) = self.map.try_idx(Position { x, y: y + 1 }) {
                if self.map.tiles[idx as usize] == TileType::Void {
                    self.map.tiles[idx as usize] = TileType::Wall;
                }
            }
        }
    }

    fn apply_vertical_tunnel_walls(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Position { x, y }) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
            if let Some(idx) = self.map.try_idx(Position { x: x - 1, y }) {
                if self.map.tiles[idx as usize] == TileType::Void {
                    self.map.tiles[idx as usize] = TileType::Wall;
                }
            }
            if let Some(idx) = self.map.try_idx(Position { x: x + 1, y }) {
                if self.map.tiles[idx as usize] == TileType::Void {
                    self.map.tiles[idx as usize] = TileType::Wall;
                }
            }
        }
    }

    fn build_corridors(&mut self) {
        let mut rng = rand::rng();
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i - 1].center();
            let new = room.center();

            if rng.random_range(0..2) == 1 {
                self.apply_horizontal_tunnel_walls(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel_walls(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel_walls(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel_walls(prev.x, new.x, new.y);
            }
        }
    }
}

fn draw_map(mut commands: Commands, mb: Res<MapBuilder>) {
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let idx = map_idx(x, y);
            match mb.map.tiles[idx] {
                TileType::Floor => {
                    commands
                        .spawn_empty()
                        .insert(Renderable {
                            glyph: to_cp437('.'),
                            fg: RGB::named(DARKOLIVEGREEN),
                            bg: RGB::named(BLACK),
                        })
                        .insert(Position { x, y });
                }
                TileType::Wall => {
                    commands
                        .spawn_empty()
                        .insert(Renderable {
                            glyph: to_cp437('.'),
                            fg: RGB::named(DARKOLIVEGREEN1),
                            bg: RGB::named(BLACK),
                        })
                        .insert(Position { x, y });
                }
                TileType::Void => {}
            }
        }
    }
}

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MapBuilder::new())
            .add_systems(Startup, draw_map);
    }
}
