use crate::prelude::*;
use bracket_lib::terminal::RGB;

#[derive(Component)]
pub struct Renderable {
    pub glyph: u16,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Copy, Clone, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Copy, Clone)]
pub struct Player {}

#[derive(Component, Copy, Clone)]
pub struct Monster {}

#[derive(Component, Debug)]
pub struct Named(pub String);
