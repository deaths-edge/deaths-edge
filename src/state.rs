use bevy::prelude::*;
use heron::PhysicsPlugin;

use crate::{
    character::CharacterPlugins,
    effects::EffectPlugin,
    environment::EnvironmentPlugin,
    game_camera::GameCameraPlugin,
    input_mapping::InputMapPlugin,
    spawning::SpawnPlugin,
    spells::SpellPlugin,
    ui::{splash::SplashUIPlugin, UIPlugins},
};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Splash,
    Lobby,
    Arena,
}

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(SplashUIPlugin);
    }
}

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CharacterPlugins)
            .add_plugins(UIPlugins)
            .add_plugin(SpawnPlugin)
            .add_plugin(InputMapPlugin)
            .add_plugin(SpellPlugin)
            .add_plugin(EffectPlugin)
            .add_plugin(PhysicsPlugin::default())
            .add_plugin(GameCameraPlugin)
            .add_plugin(EnvironmentPlugin);
    }
}
