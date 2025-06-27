use bevy::prelude::*;
use minesweeper::GamePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(
            0.0941176471,
            0.0941176471,
            0.1450980392,
        )))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Minesweeper".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(GamePlugin)
        .run();
}
