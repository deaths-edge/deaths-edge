use bevy::prelude::*;

pub struct EnvironmentMarker;

pub fn spawn_environment(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 1.0, 0.5).into()),
            sprite: Sprite::new(Vec2::new(100.0, 100.0)),
            transform: Transform::from_xyz(300., 300., 0.),
            ..Default::default()
        })
        .insert(EnvironmentMarker);

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 1.0, 0.5).into()),
            sprite: Sprite::new(Vec2::new(100.0, 100.0)),
            transform: Transform::from_xyz(-300., -300., 0.),
            ..Default::default()
        })
        .insert(EnvironmentMarker);
}
