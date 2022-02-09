pub mod button;
pub mod camera;
pub mod character_selection;
pub mod mouse;
pub mod nameplate;
pub mod selected;
pub mod splash;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

pub struct UIFonts {
    splash: Handle<Font>,
    character_selection: Handle<Font>,
}

impl FromWorld for UIFonts {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let splash = asset_server.load("fonts/Oswald-Regular.ttf");
        let character_selection = asset_server.load("fonts/Oswald-Regular.ttf");
        Self {
            splash,
            character_selection,
        }
    }
}

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .init_resource::<UIFonts>()
            .add_plugin(camera::UICameraPlugin)
            .add_plugin(mouse::WorldMousePlugin)
            .add_plugin(nameplate::NameplatePlugin)
            .add_plugin(selected::SelectedPlugin)
            .add_plugin(button::ButtonsPlugin)
            .add_plugin(character_selection::CharacterSelectionPlugin);
    }
}
