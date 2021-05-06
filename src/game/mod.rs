use bevy::prelude::*;

use animation::{AnimationPlugin, AnimationTimer};
use bullet::{Bullet, BulletPlugin};
use collision::CollisionPlugin;
use enemy::{Enemy, EnemyPlugin};
use input::InputPlugin;
use level::LevelPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;
use starfield::{Star, StarfieldPlugin};
use ui::UiPlugin;

mod animation;
mod bullet;
mod collision;
mod enemy;
mod input;
mod level;
mod physics;
mod player;
mod starfield;
mod ui;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(AnimationPlugin)
            .add_plugin(BulletPlugin)
            .add_plugin(CollisionPlugin)
            .add_plugin(EnemyPlugin)
            .add_plugin(InputPlugin)
            .add_plugin(LevelPlugin)
            .add_plugin(PhysicsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(StarfieldPlugin)
            .add_plugin(UiPlugin)
            .add_state(GameState::Playing)
            .add_system_set(
                SystemSet::on_exit(GameState::GameOver).with_system(despawn_everything.system()),
            )
            .add_startup_system(setup.system());
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum GameState {
    GameOver,
    Paused,
    Playing,
}

#[derive(Debug)]
pub struct SpriteScale {
    pub scale: f32,
}

impl SpriteScale {
    /// Create a new sprite scale.
    pub fn new(scale: f32) -> Self {
        Self { scale }
    }

    /// Get a transform translated to a coordinate.
    pub fn translate(&self, translation: Vec3) -> Transform {
        let mut transform = Transform::from_translation(translation);
        transform.scale = Vec3::new(self.scale, self.scale, 0.0);
        transform
    }

    /// Get a transform translated to a coordinate.
    pub fn xyz(&self, x: f32, y: f32, z: f32) -> Transform {
        let mut transform = Transform::from_xyz(x, y, z);
        transform.scale = Vec3::new(self.scale, self.scale, 0.0);
        transform
    }
}

#[derive(Debug)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32,
}

impl WindowSize {
    /// Get window size from existing window.
    pub fn from_window(window: &Window) -> Self {
        Self {
            width: window.width(),
            height: window.height(),
        }
    }
}

fn despawn_everything(
    mut commands: Commands,
    query: Query<Entity, Or<(With<AnimationTimer>, With<Bullet>, With<Enemy>, With<Star>)>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn setup(mut commands: Commands, windows: Res<Windows>) {
    // Set up cameras.
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Set up sprite scale.
    commands.insert_resource(SpriteScale::new(1.5));

    // Set up window size.
    commands.insert_resource({
        let window = windows.get_primary().unwrap();
        WindowSize::from_window(window)
    });
}
