pub mod camera;
pub mod mouse;
pub mod nameplate;
pub mod selected;

use bevy::{app::PluginGroupBuilder, prelude::*};

pub struct UIPlugins;

impl PluginGroup for UIPlugins {
    fn build(&mut self, group: &mut PluginGroupBuilder) {
        group
            .add(camera::UICameraPlugin)
            .add(mouse::WorldMousePlugin)
            .add(nameplate::NameplatePlugin)
            .add(selected::SelectedPlugin);
    }
}
