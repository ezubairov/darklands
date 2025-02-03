use crate::prelude::*;
use bracket_lib::terminal::RGB;

#[derive(Component)]
pub struct Renderable {
    pub glyph: u16,
    pub fg: RGB,
    pub bg: RGB,
}
