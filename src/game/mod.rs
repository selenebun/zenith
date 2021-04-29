use bevy::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Playing);
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum GameState {
    Playing,
}
