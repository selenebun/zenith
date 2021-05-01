use bevy::prelude::*;
use bevy::utils::Duration;

use crate::game::collision::DespawnOutside;
use crate::game::physics::Velocity;
use crate::game::SpriteScale;

#[derive(Bundle)]
pub struct BulletBundle {
    pub bullet: Bullet,
    pub despawn_outside: DespawnOutside,
    #[bundle]
    pub sprite: SpriteBundle,
    pub velocity: Velocity,
}

#[derive(Clone, Copy, Debug)]
pub enum Bullet {
    Small,
}

impl Bullet {
    /// Spawn a group of bullets.
    pub fn spawn(
        self,
        server: &AssetServer,
        materials: &mut Assets<ColorMaterial>,
        scale: &SpriteScale,
        position: Vec2,
        base_angle: f32,
        angles: &[f32],
        speed: f32,
        z_index: f32,
    ) -> Vec<BulletBundle> {
        let url = match self {
            Self::Small => "textures/bullets/small.png",
        };

        // Get material handle.
        let material = {
            let asset = server.load(url);
            materials.add(asset.into())
        };

        angles
            .iter()
            .map(|angle| {
                self.spawn_single(
                    material.clone(),
                    scale,
                    position,
                    base_angle + angle,
                    speed,
                    z_index,
                )
            })
            .collect()
    }

    /// Spawn a single bullet.
    fn spawn_single(
        self,
        material: Handle<ColorMaterial>,
        scale: &SpriteScale,
        position: Vec2,
        angle: f32,
        speed: f32,
        z_index: f32,
    ) -> BulletBundle {
        // Calculate velocity.
        let velocity = Velocity({
            let angle = angle.to_radians();
            let vx = speed * angle.cos();
            let vy = speed * angle.sin();
            Vec2::new(vx, vy)
        });

        BulletBundle {
            bullet: self,
            despawn_outside: DespawnOutside,
            sprite: SpriteBundle {
                material,
                transform: scale.translate(position.extend(z_index)),
                ..Default::default()
            },
            velocity,
        }
    }
}

#[derive(Debug)]
pub enum FireRate {
    Regular(Timer),
}

impl FireRate {
    /// Create a new timer.
    pub fn from_seconds(seconds: f32) -> Self {
        Self::Regular(Timer::from_seconds(seconds, true))
    }

    /// Check if able to fire.
    pub fn finished(&self) -> bool {
        match self {
            Self::Regular(timer) => timer.finished(),
        }
    }

    /// Tick the timer.
    pub fn tick(&mut self, delta: Duration) {
        if let Self::Regular(timer) = self {
            timer.tick(delta);
        }
    }
}
