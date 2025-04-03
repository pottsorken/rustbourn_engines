pub mod components;
pub mod styles;
pub mod systems;

use systems::layout::*;
use bevy::prelude::*;
use crate::AppState;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
