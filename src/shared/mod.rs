use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Menu,
    Game,
}

#[derive(Resource)]
pub struct GameConfig {
    pub mine_count: u16,
    pub grid_size: (u8, u8),
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            mine_count: 40,
            grid_size: (16, 16),
        }
    }
}
