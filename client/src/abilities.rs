use bevy::prelude::*;

use common::abilities::{AbilityPlugin as CommonAbilityPlugin, Preparing, ProjectileMarker};

use crate::state::ClientState;

pub fn adjoin_projectile_sprite_bundle(
    query: Query<(Entity, &Transform), (With<ProjectileMarker>, With<Preparing>)>,

    mut commands: Commands,
) {
    for (id, transform) in query.iter() {
        info!(message = "spawned sprite projectile", ?transform);

        commands.entity(id).insert_bundle(SpriteBundle {
            transform: *transform,
            sprite: Sprite {
                color: Color::rgb(0.5, 1.0, 0.5).into(),
                custom_size: Some(Vec2::new(15., 15.)),
                ..Default::default()
            },
            ..Default::default()
        });
    }
}

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        let adjoin = SystemSet::on_update(ClientState::Arena)
            .with_system(adjoin_projectile_sprite_bundle.system());
        app.add_plugin(CommonAbilityPlugin::new(ClientState::Arena))
            .add_system_set(adjoin);
    }
}
