pub(crate) mod consts;
pub(crate) mod game;
pub(crate) mod menu;
pub(crate) mod shared;
pub(crate) mod theme;

use bevy::prelude::*;

use crate::{game::GameLogicPlugin, menu::MenuPlugin, shared::GameState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>();
        app.add_plugins(MenuPlugin);
        app.add_plugins(GameLogicPlugin);
    }
}
