use bevy::prelude::*;

use super::{
    CastBarBundle, HealthBarBundle, NameplateBundle, NameplateMaterials, NameplateParent,
    PowerBarBundle,
};

pub fn setup_nameplate(
    character_entity: Entity,

    nameplate_materials: &NameplateMaterials,

    commands: &mut Commands,
) {
    let nameplate_bundle =
        NameplateBundle::new(NameplateParent(character_entity), &nameplate_materials);
    commands
        .spawn_bundle(nameplate_bundle)
        .with_children(|commands| {
            let health_bar_bundle = HealthBarBundle::new(nameplate_materials);
            let power_bar_bundle = PowerBarBundle::new(nameplate_materials);
            let cast_bar_bundle = CastBarBundle::new(nameplate_materials);

            commands.spawn_bundle(cast_bar_bundle);
            commands.spawn_bundle(power_bar_bundle);
            commands.spawn_bundle(health_bar_bundle);
        });
}
