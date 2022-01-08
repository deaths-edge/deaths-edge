use bevy::prelude::*;

use common::abilities::{AbilityPlugin as CommonAbilityPlugin, Preparing, ProjectileMarker};

use crate::state::ClientState;

pub fn adjoin_projectile_sprite_bundle(
    query: Query<(Entity, &Transform), (With<ProjectileMarker>, With<Preparing>)>,

    mut materials: ResMut<Assets<ColorMaterial>>,

    mut commands: Commands,
) {
    for (id, transform) in query.iter() {
        error!("spawned sprite projectile");

        commands.entity(id).insert(SpriteBundle {
            transform: *transform,
            material: materials.add(Color::rgb(0., 0., 0.).into()),
            sprite: Sprite::new(Vec2::new(30., 30.)),
            ..Default::default()
        });
    }
}

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let adjoin = SystemSet::on_update(ClientState::Arena)
            .with_system(adjoin_projectile_sprite_bundle.system());
        app.add_plugin(CommonAbilityPlugin::new(ClientState::Arena))
            .add_system_set(adjoin);
    }
}
