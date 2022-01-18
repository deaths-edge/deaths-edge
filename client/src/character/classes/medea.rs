use bevy::{ecs::system::EntityCommands, prelude::*};
use common::character::medea::{HEIGHT, WIDTH};

use crate::{character::ClientCharacterBundle, ui::selected::Selected};

#[derive(Bundle)]
pub struct ClientMedea {
    #[bundle]
    base_bundle: ClientCharacterBundle,
}

const COLOR: Color = Color::MIDNIGHT_BLUE;

impl ClientMedea {
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
            selected: Selected::default(),
        };
        let medea = ClientMedea { base_bundle };
        commands.insert_bundle(medea);
    }
}
