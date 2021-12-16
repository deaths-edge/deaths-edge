mod materials;
mod player;

use bevy::prelude::*;

pub use materials::*;
pub use player::*;

use common::character::{
    Action, CastingPlugin, CharacterBundle as CommonCharacterBundle, CharacterCommand,
    CharacterCommandPlugin, Motion,
};

use crate::{input_mapping::InputCommand, state::ClientState, ui::selected::Selected};

#[derive(Bundle)]
pub struct CharacterBundle {
    #[bundle]
    sprite: SpriteBundle,
    #[bundle]
    common: CommonCharacterBundle,
    selected: Selected,
}

impl CharacterBundle {
    pub fn new(
        transform: Transform,
        common: CommonCharacterBundle,
        materials: &CharacterMaterials,
    ) -> Self {
        let size = common.class().size();

        Self {
            sprite: SpriteBundle {
                material: materials.handle(common.class()).clone(),
                transform,
                sprite: Sprite::new(Vec2::new(size.width, size.width)),
                ..Default::default()
            },
            common,
            selected: Selected::default(),
        }
    }
}

fn input_to_character<Value>(
    mut input_motion: EventReader<InputCommand<Value>>,
    mut command_motion: EventWriter<CharacterCommand<Value>>,
    player_query: Query<Entity, With<PlayerMarker>>,
) where
    Value: Clone + Send + Sync + 'static,
{
    let entity = player_query.single().expect("missing player");
    command_motion.send_batch(
        input_motion
            .iter()
            .map(|input| CharacterCommand::new(entity, input.action().clone())),
    )
}

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let input_to_character = SystemSet::on_update(PlayerState::Spawned)
            .label("input-to-character")
            .with_system(input_to_character::<Motion>.system())
            .with_system(input_to_character::<Action>.system());
        app.add_system_set(input_to_character)
            .add_plugin(CharacterCommandPlugin::new(PlayerState::Spawned))
            .add_plugin(CharacterMaterialPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(CastingPlugin::new(PlayerState::Spawned));
    }
}
