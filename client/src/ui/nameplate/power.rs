use bevy::prelude::*;
use common::character::{CharacterMarker, Power};

use super::{NameplateMarker, NameplateMaterials, NameplateParent};

#[derive(Debug)]
pub struct PowerBarMarker;

#[derive(Debug, Bundle)]
pub struct PowerBarBundle {
    marker: PowerBarMarker,
    #[bundle]
    node: NodeBundle,
}

impl PowerBarBundle {
    pub fn new(nameplate_materials: &NameplateMaterials) -> Self {
        Self {
            marker: PowerBarMarker,
            node: NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    ..Default::default()
                },
                material: nameplate_materials.energy_bar.clone(),
                ..Default::default()
            },
        }
    }
}

pub fn power_bar_update(
    mut healthbar_query: Query<(&Parent, &mut Style), With<PowerBarMarker>>,
    nameplate_query: Query<&NameplateParent, With<NameplateMarker>>,
    character_query: Query<&Power, (With<CharacterMarker>, Changed<Power>)>,
) {
    for (healthbar_parent, mut healthbar_style) in healthbar_query.iter_mut() {
        if let Ok(nameplate_parent) = nameplate_query.get(healthbar_parent.0) {
            if let Ok(character_power) = character_query.get(nameplate_parent.0) {
                let percent = 100. * character_power.current as f32 / character_power.total as f32;
                info!(%percent, current = ?healthbar_style.margin.right);
                healthbar_style.size.width = Val::Percent(percent);
            }
        }
    }
}
