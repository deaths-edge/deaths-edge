use bevy::prelude::*;

use super::NameplateMaterials;

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
                    size: Size::new(Val::Percent(100.), Val::Percent(50.)),
                    ..Default::default()
                },
                material: nameplate_materials.primary_health_bar.clone(),
                ..Default::default()
            },
        }
    }
}
