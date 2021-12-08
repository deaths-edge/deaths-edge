use bevy::prelude::*;

use crate::{
    character::CharacterPlugin,
    game_camera::GameCameraPlugin,
    input_mapping::InputMapPlugin,
    music::SplashMusicPlugin,
    spawning::SpawnPlugin,
    ui::{splash::SplashUIPlugin, UIPlugins},
};

use common::{
    effects::EffectPlugin, environment::EnvironmentPlugin, heron::PhysicsPlugin,
    spells::SpellPlugin,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ClientState {
    Splash,
    Lobby,
    Arena,
}

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(SplashUIPlugin).add_plugin(SplashMusicPlugin);
    }
}

pub struct ArenaPlugin;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(CharacterPlugin)
            .add_plugins(UIPlugins)
            .add_plugin(SpawnPlugin)
            .add_plugin(InputMapPlugin)
            .add_plugin(SpellPlugin::new(ClientState::Arena))
            .add_plugin(EffectPlugin::new(ClientState::Arena))
            .add_plugin(PhysicsPlugin::default())
            .add_plugin(GameCameraPlugin)
            .add_plugin(EnvironmentPlugin::new(ClientState::Arena));
    }
}
