use bevy::prelude::*;

use crate::character::CharacterIndex;

pub struct SelectedPlugin;

impl Plugin for SelectedPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(select_highlight.system());
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Selected {
    Selected,
    Unselected,
}

impl Selected {
    pub fn is_selected(&self) -> bool {
        *self == Selected::Selected
    }
}

impl Default for Selected {
    fn default() -> Self {
        Self::Unselected
    }
}

pub fn select_highlight(
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<
        (&mut Handle<ColorMaterial>, &Selected),
        (Changed<Selected>, With<CharacterIndex>),
    >,
) {
    for (mut material, selected) in query.iter_mut() {
        if selected.is_selected() {
            *material = materials.add(Color::OLIVE.into())
        } else {
            *material = materials.add(Color::MAROON.into())
        }
    }
}
