use bevy::prelude::*;

use super::NameplateMaterials;

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
