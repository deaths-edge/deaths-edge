use bevy::prelude::*;

use super::{CastBarBundle, HealthBarBundle, NameplateBundle, NameplateMaterials, PowerBarBundle};

pub fn setup_nameplate(
    character_entity: In<Entity>,

    nameplate_materials: Res<NameplateMaterials>,

    mut commands: Commands,
) {
    let nameplate_bundle = NameplateBundle::new(character_entity.0.into(), &nameplate_materials);
    commands
        .spawn_bundle(nameplate_bundle)
        .with_children(|commands| {
            let health_bar_bundle = HealthBarBundle::new(&nameplate_materials);
            let power_bar_bundle = PowerBarBundle::new(&nameplate_materials);
            let cast_bar_bundle = CastBarBundle::new(&nameplate_materials);

            commands.spawn_bundle(cast_bar_bundle);
            commands.spawn_bundle(power_bar_bundle);
            commands.spawn_bundle(health_bar_bundle);
        });
}
