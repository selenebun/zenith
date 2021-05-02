use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

#[derive(Debug)]
pub struct HealthBar;

fn setup(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Get material handle.
    let material = {
        let asset = server.load("textures/ui/heart.png");
        materials.add(asset.into())
    };

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::SpaceBetween,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        align_content: AlignContent::FlexEnd,
                        ..Default::default()
                    },
                    material: materials.add(Color::NONE.into()),
                    ..Default::default()
                })
                .insert(HealthBar)
                .with_children(|parent| {
                    for _ in 0..5 {
                        parent.spawn_bundle(ImageBundle {
                            style: Style {
                                margin: Rect {
                                    left: Val::Px(20.0),
                                    bottom: Val::Px(20.0),
                                    ..Default::default()
                                },
                                size: Size::new(Val::Px(24.0), Val::Px(21.0)),
                                ..Default::default()
                            },
                            material: material.clone(),
                            transform: Transform::from_scale(Vec3::new(1.5, 1.5, 0.0)),
                            ..Default::default()
                        });
                    }
                });
        });
}
