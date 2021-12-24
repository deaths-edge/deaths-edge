use bevy::prelude::*;

use common::character::{CharacterCastState, CharacterMarker};

use super::{NameplateMarker, NameplateMaterials, NameplateParent};

#[derive(Debug)]
pub struct CastBarMarker;

#[derive(Debug, Bundle)]
pub struct CastBarBundle {
    marker: CastBarMarker,
    #[bundle]
    node: NodeBundle,
}

impl CastBarBundle {
    pub fn new(nameplate_materials: &NameplateMaterials) -> Self {
        Self {
            marker: CastBarMarker,
            node: NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    ..Default::default()
                },
                material: nameplate_materials.cast_bar.clone(),
                ..Default::default()
            },
        }
    }
}

pub fn cast_bar_update(
    time: Res<Time>,
    mut cast_bar_query: Query<(&Parent, &mut Style), With<CastBarMarker>>,
    nameplate_query: Query<&NameplateParent, With<NameplateMarker>>,
    character_query: Query<&CharacterCastState, With<CharacterMarker>>,
) {
    for (cast_bar_parent, mut cast_bar_style) in cast_bar_query.iter_mut() {
        if let Ok(nameplate_parent) = nameplate_query.get(cast_bar_parent.0) {
            if let Ok(character_cast_state) = character_query.get(nameplate_parent.id()) {
                if let Some(character_cast) = character_cast_state.cast() {
                    cast_bar_style.display = Display::Flex;
                    if let Some(cast_total_duration) = character_cast.spell.duration() {
                        let now = time.last_update().expect("last update not found");
                        let current_duration = now - character_cast.start;

                        let percent = 100. * current_duration.as_secs_f32()
                            / cast_total_duration.as_secs_f32();

                        cast_bar_style.size.width = Val::Percent(percent);
                    } else {
                        cast_bar_style.display = Display::None;
                    }
                } else {
                    cast_bar_style.display = Display::None;
                }
            }
        }
    }
}
