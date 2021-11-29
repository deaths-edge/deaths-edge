mod health;
mod materials;
mod power;

use bevy::prelude::*;

pub use health::*;
pub use materials::*;
pub use power::*;

#[derive(Debug)]
pub struct NameplateMarker;

#[derive(Debug, Bundle)]
pub struct NameplateBundle {
    marker: NameplateMarker,
    #[bundle]
    node: NodeBundle,
}

impl NameplateBundle {
    pub fn new(nameplate_materials: &NameplateMaterials) -> Self {
        Self {
            marker: NameplateMarker,
            node: NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(7.5), Val::Percent(2.5)),
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    position: Rect {
                        left: Val::Px(400.0),
                        bottom: Val::Px(300.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                material: nameplate_materials.primary_health_bar.clone(),
                ..Default::default()
            },
        }
    }
}
