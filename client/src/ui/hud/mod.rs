use bevy::prelude::Plugin;

pub mod button;
pub mod camera;
pub mod nameplate;
pub mod selected;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HudState {
    Active,
    Inactive,
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_state(HudState::Inactive)
            .add_plugin(nameplate::NameplatePlugin)
            .add_plugin(selected::SelectedPlugin)
            .add_plugin(button::ButtonsPlugin)
            .add_plugin(camera::UICameraPlugin);
    }
}
