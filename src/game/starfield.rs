use bevy::prelude::*;
use rand::prelude::*;

use crate::game::physics::Velocity;
use crate::game::{GameState, WindowSize};

pub struct StarfieldPlugin;

impl Plugin for StarfieldPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(setup.system()));
    }
}

#[derive(Bundle)]
pub struct StarBundle {
    pub star: Star,
    #[bundle]
    pub sprite: SpriteBundle,
    pub velocity: Velocity,
}

#[derive(Debug)]
pub struct Star;

fn setup(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Res<WindowSize>,
) {
    // Get material handles.
    let urls = vec![
        ("textures/starfield/small.png", 3),
        ("textures/starfield/medium.png", 2),
        ("textures/starfield/large.png", 1),
    ];
    let materials: Vec<_> = urls
        .iter()
        .map(|(url, weight)| {
            let asset = server.load(*url);
            (materials.add(asset.into()), weight)
        })
        .collect();

    // Create starfield.
    let mut rng = rand::thread_rng();
    for _ in 0..200 {
        // Get material handle.
        let material = materials
            .choose_weighted(&mut rng, |e| e.1)
            .unwrap()
            .0
            .clone();

        // Calculate transform.
        let transform = {
            let x = rng.gen_range((-window.width / 2.0)..(window.width / 2.0));
            let y = rng.gen_range((-window.height / 2.0)..(window.height / 2.0));
            Transform::from_translation(Vec3::new(x, y, 0.0))
        };

        // Calculate velocity.
        let velocity = {
            let speed = rng.gen_range(1.0..9.0);
            Velocity(Vec2::new(0.0, -speed))
        };

        commands.spawn_bundle(StarBundle {
            star: Star,
            sprite: SpriteBundle {
                material,
                transform,
                ..Default::default()
            },
            velocity,
        });
    }
}
