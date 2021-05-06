use bevy::prelude::*;
use bevy::utils::Duration;
use rand::prelude::*;

use crate::game::animation;
use crate::game::collision::{self, DespawnOutside, Hitbox};
use crate::game::enemy::EnemyFaction;
use crate::game::physics::{Acceleration, Velocity};
use crate::game::{GameState, SpriteScale, WindowSize};

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(floor_behavior.system())
                .with_system(wall_behavior.system()),
        );
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Bullet {
    Basic,
    Bomb,
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
        base_velocity: Vec2,
        acceleration: Vec2,
        base_angle: f32,
        angles: &[f32],
        speed: f32,
        z_index: f32,
    ) -> Vec<BulletBundle> {
        let url = match self {
            Self::Basic => "textures/bullets/basic.png",
            Self::Bomb => "textures/bullets/bomb.png",
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
                    base_velocity,
                    acceleration,
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
        base_velocity: Vec2,
        acceleration: Vec2,
        angle: f32,
        speed: f32,
        z_index: f32,
    ) -> BulletBundle {
        let (damage, radius, wall_behavior, floor_behavior) = match self {
            Self::Basic => (1, 3.0, WallBehavior::None, FloorBehavior::None),
            Self::Bomb => (1, 4.0, WallBehavior::Bounce, FloorBehavior::Explode),
            Self::Small => (1, 1.0, WallBehavior::None, FloorBehavior::None),
        };

        // Calculate velocity.
        let velocity = Velocity({
            let angle = angle.to_radians();
            let vx = speed * angle.cos();
            let vy = speed * angle.sin();
            Vec2::new(vx, vy) + base_velocity
        });

        BulletBundle {
            acceleration: Acceleration(acceleration),
            bullet: self,
            damage: Damage(damage),
            despawn_outside: DespawnOutside,
            floor_behavior,
            hitbox: Hitbox {
                radius: radius * scale.scale,
            },
            sprite: SpriteBundle {
                material,
                transform: scale.translate(position.extend(z_index)),
                ..Default::default()
            },
            velocity,
            wall_behavior,
        }
    }
}

#[derive(Bundle)]
pub struct BulletBundle {
    pub acceleration: Acceleration,
    pub bullet: Bullet,
    pub despawn_outside: DespawnOutside,
    pub damage: Damage,
    pub floor_behavior: FloorBehavior,
    pub hitbox: Hitbox,
    #[bundle]
    pub sprite: SpriteBundle,
    pub velocity: Velocity,
    pub wall_behavior: WallBehavior,
}

#[derive(Debug)]
pub struct Damage(pub u32);

#[derive(Debug)]
pub enum FireRate {
    Random(f32),
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
            Self::Random(chance) => {
                let mut rng = rand::thread_rng();
                rng.gen::<f32>() < *chance
            }
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

#[derive(Debug)]
pub enum FloorBehavior {
    Explode,
    None,
}

#[derive(Debug)]
pub enum WallBehavior {
    Bounce,
    None,
}

fn floor_behavior(
    mut commands: Commands,
    server: Res<AssetServer>,
    audio: Res<Audio>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    scale: Res<SpriteScale>,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    window: Res<WindowSize>,
    query: Query<(Entity, &FloorBehavior, &Sprite, &Transform), With<Bullet>>,
) {
    for (entity, floor_behavior, sprite, transform) in query.iter() {
        match floor_behavior {
            FloorBehavior::Explode => {
                // Explode on contact with floor.
                let height =
                    collision::inner_bound(window.height, sprite.size.y * transform.scale.y);
                if transform.translation.y < -height {
                    commands.entity(entity).despawn();
                    commands.spawn_bundle(animation::spawn_explosion(
                        &server,
                        &audio,
                        &mut atlases,
                        *transform,
                    ));

                    // Spawn bullets.
                    for bullet in Bullet::Basic.spawn(
                        &server,
                        &mut materials,
                        &scale,
                        transform.translation.truncate(),
                        Vec2::ZERO,
                        Vec2::ZERO,
                        90.0,
                        &[-45.0, 0.0, 45.0],
                        8.0,
                        4.0,
                    ) {
                        commands.spawn_bundle(bullet).insert(EnemyFaction);
                    }
                }
            }
            FloorBehavior::None => {}
        }
    }
}

fn wall_behavior(
    window: Res<WindowSize>,
    mut query: Query<(&Sprite, &mut Transform, &mut Velocity, &WallBehavior), With<Bullet>>,
) {
    for (sprite, mut transform, mut velocity, wall_behavior) in query.iter_mut() {
        match wall_behavior {
            WallBehavior::Bounce => {
                // Change directions when hitting wall.
                let width = collision::inner_bound(window.width, sprite.size.x * transform.scale.x);
                if transform.translation.x > width || transform.translation.x < -width {
                    transform.translation.x = transform.translation.x.min(width).max(-width);
                    velocity.0.x *= -1.0;
                }
            }
            WallBehavior::None => {}
        }
    }
}
