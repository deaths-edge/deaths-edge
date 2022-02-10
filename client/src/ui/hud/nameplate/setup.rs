use bevy::prelude::*;

use super::{CastBarBundle, HealthBarBundle, NameplateBundle, NameplateParent, PowerBarBundle};

pub fn setup_nameplate(character_entity: Entity, commands: &mut Commands) {
    let nameplate_bundle = NameplateBundle::new(NameplateParent(character_entity));
    commands
        .spawn_bundle(nameplate_bundle)
        .with_children(|commands| {
            let health_bar_bundle = HealthBarBundle::new();
            let power_bar_bundle = PowerBarBundle::new();
            let cast_bar_bundle = CastBarBundle::new();

            commands.spawn_bundle(cast_bar_bundle);
            commands.spawn_bundle(power_bar_bundle);
            commands.spawn_bundle(health_bar_bundle);
        });
}
