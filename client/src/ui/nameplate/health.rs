use bevy::prelude::*;

use common::character::{CharacterMarker, Health};

use super::{NameplateMarker, NameplateParent};

#[derive(Debug, Component)]
pub struct HealthBarMarker;

#[derive(Debug, Bundle)]
pub struct HealthBarBundle {
    marker: HealthBarMarker,
    #[bundle]
    node: NodeBundle,
}

impl HealthBarBundle {
    pub fn new() -> Self {
        Self {
            marker: HealthBarMarker,
            node: NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    ..Default::default()
                },
                color: Color::GREEN.into(),
                ..Default::default()
            },
        }
    }
}

pub fn health_bar_update(
    mut healthbar_query: Query<(&Parent, &mut Style), With<HealthBarMarker>>,
    nameplate_query: Query<&NameplateParent, With<NameplateMarker>>,
    character_query: Query<&Health, (With<CharacterMarker>, Changed<Health>)>,
) {
    for (healthbar_parent, mut healthbar_style) in healthbar_query.iter_mut() {
        if let Ok(nameplate_parent) = nameplate_query.get(healthbar_parent.0) {
            if let Ok(character_health) = character_query.get(nameplate_parent.0) {
                let percent =
                    100. * character_health.current as f32 / character_health.total as f32;
                healthbar_style.size.width = Val::Percent(percent);
            }
        }
    }
}
