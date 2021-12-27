use bevy::prelude::*;

use super::CharacterSelectionMaterials;

pub struct CharacterConfirmButtonMarker;

#[derive(Bundle)]
pub struct CharacterConfirmButton {
    marker: CharacterConfirmButtonMarker,
    #[bundle]
    button: ButtonBundle,
}

impl CharacterConfirmButton {
    pub fn new(materials: &CharacterSelectionMaterials) -> Self {
        Self {
            marker: CharacterConfirmButtonMarker,
            button: ButtonBundle {
                style: Style {
                    position_type: PositionType::Relative,
                    size: Size::new(Val::Px(65.0), Val::Px(35.0)),
                    // margin: Rect::all(Val::Auto),
                    // justify_content: JustifyContent::Center,
                    // align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: materials.confirm_button.clone(),
                ..Default::default()
            },
        }
    }
}
