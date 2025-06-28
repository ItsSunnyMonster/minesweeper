use bevy::prelude::*;

trait CatppuccinColorToBevyColorExt {
    fn to_bevy_color(&self) -> Color;
}

impl CatppuccinColorToBevyColorExt for catppuccin::Color {
    fn to_bevy_color(&self) -> Color {
        Color::hsl(self.hsl.h as f32, self.hsl.s as f32, self.hsl.l as f32)
    }
}

impl From<catppuccin::FlavorColors> for super::ColorTheme {
    fn from(value: catppuccin::FlavorColors) -> Self {
        Self {
            background: value.mantle.to_bevy_color(),

            closed_tile: value.surface0.to_bevy_color(),
            hovered_tile: value.surface1.to_bevy_color(),
            opened_tile: value.base.to_bevy_color(),
            flagged_tile: value.overlay2.to_bevy_color(),
            exposed_mine_tile: value.red.to_bevy_color(),

            one: value.sky.to_bevy_color(),
            two: value.green.to_bevy_color(),
            three: value.red.to_bevy_color(),
            four: value.blue.to_bevy_color(),
            five: value.lavender.to_bevy_color(),
            six: value.yellow.to_bevy_color(),
            seven: value.pink.to_bevy_color(),
            eight: value.mauve.to_bevy_color(),

            mine: value.mantle.to_bevy_color(),
            flag: value.mantle.to_bevy_color(),
        }
    }
}
