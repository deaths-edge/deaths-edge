use bevy::prelude::*;

use common::{
    abilities::{AbilityMarker, CastType},
    character::{CastState, CharacterMarker},
};

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
    ability_query: Query<&CastType, With<AbilityMarker>>,
    nameplate_query: Query<&NameplateParent, With<NameplateMarker>>,
    character_query: Query<&CastState, With<CharacterMarker>>,
) {
    for (cast_bar_parent, mut cast_bar_style) in cast_bar_query.iter_mut() {
        if let Ok(nameplate_parent) = nameplate_query.get(cast_bar_parent.0) {
            if let Ok(character_cast_state) = character_query.get(nameplate_parent.0) {
                if let Some(character_cast) = &character_cast_state.0 {
                    cast_bar_style.display = Display::Flex;

                    let cast_type = ability_query
                        .get(character_cast.ability_id)
                        .expect("could not find ability");

                    match cast_type {
                        CastType::Instant => continue,
                        CastType::Cast(cast_total_duration) => {
                            let now = time.last_update().expect("last update not found");
                            let current_duration = now - character_cast.start;

                            let percent = 100. * current_duration.as_secs_f32()
                                / cast_total_duration.as_secs_f32();

                            cast_bar_style.size.width = Val::Percent(percent);
                        }
                        CastType::Channel(_) => todo!(),
                    }
                } else {
                    cast_bar_style.display = Display::None;
                }
            }
        }
    }
}
