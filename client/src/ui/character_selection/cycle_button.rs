use bevy::prelude::*;
use common::character::CharacterClass;

use super::CharacterSelectionMaterials;

pub struct CharacterCycleButtonMarkerLeft;

pub struct CharacterCycleButtonMarkerRight;

#[derive(Bundle)]
pub struct CharacterCycleButton<T>
where
    T: Send + Sync + 'static,
{
    marker: T,
    #[bundle]
    button: ButtonBundle,
}

impl<T> CharacterCycleButton<T>
where
    T: Send + Sync + 'static,
{
    pub fn new(marker: T, materials: &CharacterSelectionMaterials) -> Self {
        Self {
            marker,
            button: ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(65.0), Val::Px(35.0)),
                    // margin: Rect::all(Val::Auto),
                    // justify_content: JustifyContent::Center,
                    // align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: materials.cycle_button.clone(),
                ..Default::default()
            },
        }
    }
}

impl CharacterCycleButton<CharacterCycleButtonMarkerLeft> {
    pub fn new_left(materials: &CharacterSelectionMaterials) -> Self {
        Self::new(CharacterCycleButtonMarkerLeft, materials)
    }
}

impl CharacterCycleButton<CharacterCycleButtonMarkerRight> {
    pub fn new_right(materials: &CharacterSelectionMaterials) -> Self {
        Self::new(CharacterCycleButtonMarkerRight, materials)
    }
}

pub fn handle_click_left(
    mut interaction_query: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<Button>,
            With<CharacterCycleButtonMarkerLeft>,
        ),
    >,
    mut selected_char: ResMut<CharacterClass>,
) {
    for interaction in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *selected_char = selected_char.cycle_left();
            }
            _ => (),
        }
    }
}

pub fn handle_click_right(
    mut interaction_query: Query<
        &Interaction,
        (
            Changed<Interaction>,
            With<Button>,
            With<CharacterCycleButtonMarkerRight>,
        ),
    >,
    mut selected_char: ResMut<CharacterClass>,
) {
    for interaction in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *selected_char = selected_char.cycle_right();
            }
            _ => (),
        }
    }
}
