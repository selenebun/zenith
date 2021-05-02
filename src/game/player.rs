use bevy::prelude::*;

use crate::game::animation::AnimationTimer;
use crate::game::bullet::FireRate;
use crate::game::collision::SpriteSize;
use crate::game::enemy::Health;
use crate::game::ui::HealthBar;
use crate::game::{GameState, SpriteScale, WindowSize};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(spawn_player.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(update_health_bar.system()),
        );
    }
}

#[derive(Debug)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub fire_rate: FireRate,
    pub health: Health,
    pub player: Player,
    pub speed: Speed,
    #[bundle]
    pub sprite: SpriteSheetBundle,
    pub sprite_size: SpriteSize,
    pub timer: AnimationTimer,
}

#[derive(Debug)]
pub struct PlayerFaction;

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
        fire_rate: FireRate::from_seconds(0.18),
        health: Health::new(5),
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

fn update_health_bar(
    server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    health_bar: Query<&Children, With<HealthBar>>,
    mut hearts: Query<&mut Handle<ColorMaterial>>,
    player: Query<&Health, (With<Player>, Changed<Health>)>,
) {
    // Get player health if it has changed.
    let health = match player.iter().next() {
        Some(health) => health,
        None => return,
    };

    // Get material handles.
    let full_heart = {
        let asset = server.load("textures/ui/heart.png");
        materials.add(asset.into())
    };
    let empty_heart = {
        let asset = server.load("textures/ui/heart_empty.png");
        materials.add(asset.into())
    };

    for (i, heart) in health_bar
        .single()
        .expect("expected a single health bar")
        .iter()
        .enumerate()
    {
        let mut material = hearts.get_mut(*heart).expect("expected a single heart");
        *material = if i < health.current as usize {
            full_heart.clone()
        } else {
            empty_heart.clone()
        };
    }
}
