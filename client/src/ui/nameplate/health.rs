use bevy::prelude::*;

use common::character::{CharacterHealth, CharacterMarker};

use super::{NameplateMarker, NameplateMaterials, NameplateParent};

#[derive(Debug)]
pub struct HealthBarMarker;

#[derive(Debug, Bundle)]
pub struct HealthBarBundle {
    marker: HealthBarMarker,
    #[bundle]
    node: NodeBundle,
}

impl HealthBarBundle {
    pub fn new(nameplate_materials: &NameplateMaterials) -> Self {
        Self {
            marker: HealthBarMarker,
            node: NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    ..Default::default()
                },
                material: nameplate_materials.health_bar.clone(),
                ..Default::default()
            },
        }
    }
}

pub fn health_bar_update(
    mut healthbar_query: Query<(&Parent, &mut Style), With<HealthBarMarker>>,
    nameplate_query: Query<&NameplateParent, With<NameplateMarker>>,
    character_query: Query<&CharacterHealth, (With<CharacterMarker>, Changed<CharacterHealth>)>,
) {
    for (healthbar_parent, mut healthbar_style) in healthbar_query.iter_mut() {
        if let Ok(nameplate_parent) = nameplate_query.get(healthbar_parent.0) {
            if let Ok(character_health) = character_query.get(nameplate_parent.id()) {
                let percent =
                    100. * character_health.current as f32 / character_health.total as f32;
                info!(%percent, current = ?healthbar_style.margin.right);
                healthbar_style.size.width = Val::Percent(percent);
            }
        }
    }
}
