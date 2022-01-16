use bevy::prelude::*;

use crate::state::ClientState;

pub struct Music {
    splash: Handle<AudioSource>,
}

impl FromWorld for Music {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.get_resource::<AssetServer>().unwrap();
        let splash = asset_server.load("sounds/music/passaggio_arr_for_two_cellos.mp3");
        Self { splash }
    }
}

fn setup(asset_server: Res<Music>, audio: Res<Audio>) {
    // audio.play(asset_server.splash.clone());
}

fn fade_splash(asset_server: Res<Music>, audio: Res<Audio>) {
    // TODO: Fade out music
}

pub struct SplashMusicPlugin;

impl Plugin for SplashMusicPlugin {
    fn build(&self, app: &mut App) {
        let splash_music = SystemSet::on_enter(ClientState::Arena).with_system(fade_splash);
        app.init_resource::<Music>().add_startup_system(setup);
    }
}
