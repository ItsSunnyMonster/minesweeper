pub(crate) mod catppuccin;

use bevy::prelude::*;

#[derive(Resource)]
#[allow(dead_code)] // TODO: Remove once all used
pub struct ColorTheme {
    // UI
    pub background: Color,

    // Tiles
    pub closed_tile: Color,
    pub hovered_tile: Color,
    pub opened_tile: Color,
    pub flagged_tile: Color,
    pub exposed_mine_tile: Color,

    // Numbers
    pub one: Color,
    pub two: Color,
    pub three: Color,
    pub four: Color,
    pub five: Color,
    pub six: Color,
    pub seven: Color,
    pub eight: Color,

    // Icons
    pub mine: Color,
    pub flag: Color,
}
