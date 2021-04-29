use bevy::prelude::*;

use crate::game::animation::AnimationTimer;
use crate::game::{GameState, SpriteScale, WindowSize};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(spawn_player.system()),
        );
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    #[bundle]
    pub sprite: SpriteSheetBundle,
    pub timer: AnimationTimer,
}

#[derive(Debug)]
pub struct Player;

fn spawn_player(
    mut commands: Commands,
    server: Res<AssetServer>,
    scale: Res<SpriteScale>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    window: Res<WindowSize>,
) {
    // Get texture atlas handle.
    let texture_atlas = {
        let asset = server.load("textures/player.png");
        let atlas = TextureAtlas::from_grid(asset, Vec2::new(50.0, 43.0), 4, 1);
        atlases.add(atlas)
    };

    commands.spawn_bundle(PlayerBundle {
        player: Player,
        sprite: SpriteSheetBundle {
            texture_atlas,
            transform: scale.xyz(0.0, -window.height / 4.0, 3.0),
            ..Default::default()
        },
        timer: AnimationTimer::new(0.1),
    });
}
