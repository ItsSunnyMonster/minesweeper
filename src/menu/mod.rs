use bevy::prelude::*;

use crate::{
    shared::{GameConfig, GameState},
    theme::ColorTheme,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, start_game);
    }
}

fn start_game(mut next_state: ResMut<NextState<GameState>>, mut commands: Commands) {
    // * Nothing was rendered until I realised I forgot to spawn a camera.
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 0.0),
        // Projection::Orthographic(OrthographicProjection {
        //     scaling_mode: ScalingMode::FixedHorizontal {
        //         viewport_width: consts::WINDOW_WIDTH,
        //     },
        //     ..OrthographicProjection::default_2d()
        // }),
    ));

    // Temporary Setup
    let color_theme: ColorTheme = catppuccin::PALETTE.mocha.colors.into();
    commands.insert_resource(ClearColor(color_theme.background));
    commands.insert_resource(color_theme);
    commands.insert_resource(GameConfig::default());
    next_state.set(GameState::Game);
}
