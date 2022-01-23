use bevy::{ecs::system::EntityCommands, prelude::*};
use common::character::mars::{HEIGHT, WIDTH};

use crate::character::ClientCharacterBundle;

#[derive(Bundle)]
pub struct ClientMars {
    #[bundle]
    base_bundle: ClientCharacterBundle,
}

const COLOR: Color = Color::TOMATO;

impl ClientMars {
    pub fn insert_bundle(commands: &mut EntityCommands) {
        let base_bundle = ClientCharacterBundle {
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: COLOR,
                    custom_size: Some(Vec2::new(WIDTH, HEIGHT)),
                    ..Default::default()
                },
                ..Default::default()
            },
        };
        let medea = ClientMars { base_bundle };
        commands.insert_bundle(medea);
    }
}
