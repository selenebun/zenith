use bevy::prelude::*;

use crate::game::animation::AnimationTimer;
use crate::game::collision::SpriteSize;
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
    pub speed: Speed,
    #[bundle]
    pub sprite: SpriteSheetBundle,
    pub sprite_size: SpriteSize,
    pub timer: AnimationTimer,
}

#[derive(Debug)]
pub struct Player;

#[derive(Debug)]
pub struct Speed(pub f32);

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

    // Get sprite size.
    let sprite_size = {
        let atlas = atlases.get(&texture_atlas).unwrap();
        let rect = atlas.textures.first().unwrap();
        SpriteSize::new(rect.width(), rect.height(), scale.scale)
    };

    commands.spawn_bundle(PlayerBundle {
        player: Player,
        speed: Speed(6.0),
        sprite: SpriteSheetBundle {
            texture_atlas,
            transform: scale.xyz(0.0, -window.height / 4.0, 3.0),
            ..Default::default()
        },
        sprite_size,
        timer: AnimationTimer::new(0.1),
    });
}
