pub mod character_selection;
pub mod hud;
pub mod mouse;
pub mod splash;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_plugin(character_selection::CharacterSelectionPlugin)
            .add_plugin(hud::HudPlugin)
            .add_plugin(mouse::WorldMousePlugin);
    }
}
