use bevy::prelude::*;

use animation::AnimationPlugin;
use collision::CollisionPlugin;
use physics::PhysicsPlugin;
use player::PlayerPlugin;
use starfield::StarfieldPlugin;

mod animation;
mod collision;
mod physics;
mod player;
mod starfield;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(AnimationPlugin)
            .add_plugin(CollisionPlugin)
            .add_plugin(PhysicsPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(StarfieldPlugin)
            .add_state(GameState::Playing)
            .add_startup_system(setup.system());
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum GameState {
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

    /// Get a transform translatied to a coordinate.
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

fn setup(mut commands: Commands, windows: Res<Windows>) {
    // Set up cameras.
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Set up sprite scale.
    commands.insert_resource(SpriteScale::new(1.5));

    // Set up window size.
    commands.insert_resource({
        let window = windows.get_primary().unwrap();
        WindowSize::from_window(window)
    });
}
