use bevy::{ecs::system::EntityCommands, prelude::*};
use common::character::dummy::{HEIGHT, WIDTH};

use crate::character::ClientCharacterBundle;

#[derive(Bundle)]
pub struct ClientDummy {
    #[bundle]
    base_bundle: ClientCharacterBundle,
}

const COLOR: Color = Color::BISQUE;

impl ClientDummy {
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
        let medea = ClientDummy { base_bundle };
        commands.insert_bundle(medea);
    }
}
