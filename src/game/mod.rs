use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    consts,
    shared::{GameConfig, GameState},
    theme::ColorTheme,
};

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup_game);
    }
}

fn setup_game(
    mut commands: Commands,
    config: Res<GameConfig>,
    color_theme: Res<ColorTheme>,
    query: Query<&Window, With<PrimaryWindow>>,
) {
    info!(
        "Config: {} mines, ({}, {})",
        config.mine_count, config.grid_size.0, config.grid_size.1
    );

    let window = query.single().unwrap();

    let window_width = window.width();
    let window_height = window.height();

    let (grid_width, grid_height) = config.grid_size;

    // Rearranged from grid_width / grid_height >= window_width / window_height
    let grid_cell_size = if grid_width as f32 * window_height >= window_width * grid_height as f32 {
        // Width is limiting factor
        window_width / (grid_width as f32 + consts::GRID_GAP_PERCENTAGE * (grid_width + 1) as f32)
    } else {
        // Height is limiting factor
        window_height
            / (grid_height as f32 + consts::GRID_GAP_PERCENTAGE * (grid_height + 1) as f32)
    };

    let gap_size = grid_cell_size * consts::GRID_GAP_PERCENTAGE;
    let half_grid_width_in_pixels =
        (grid_cell_size * grid_width as f32 + gap_size * (grid_width - 1) as f32) * 0.5;
    let half_grid_height_in_pixels =
        (grid_cell_size * grid_height as f32 + gap_size * (grid_height - 1) as f32) * 0.5;
    let half_grid_cell_size = grid_cell_size * 0.5;

    for x in 0..grid_width {
        for y in 0..grid_height {
            commands.spawn((
                Sprite::from_color(
                    color_theme.closed_tile,
                    Vec2::splat(grid_cell_size), // * Why is it called splat??
                ),
                Transform::from_xyz(
                    -half_grid_width_in_pixels
                        + half_grid_cell_size
                        + (gap_size + grid_cell_size) * x as f32,
                    -half_grid_height_in_pixels
                        + half_grid_cell_size
                        + (gap_size + grid_cell_size) * y as f32,
                    0.0,
                ),
            ));
        }
    }
}
