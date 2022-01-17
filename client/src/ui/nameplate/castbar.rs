use bevy::prelude::*;

use common::{
    abilities::{CastDuration, CastMarker},
    character::{Cast, CastState, CharacterMarker},
};

use super::{NameplateMarker, NameplateParent};

#[derive(Debug, Default, Component)]
pub struct CastBarMarker;

#[derive(Debug, Bundle)]
pub struct CastBarBundle {
    marker: CastBarMarker,
    #[bundle]
    node: NodeBundle,
}

impl CastBarBundle {
    pub fn new() -> Self {
        Self {
            marker: CastBarMarker,
            node: NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                    ..Default::default()
                },
                color: Color::YELLOW.into(),
                ..Default::default()
            },
        }
    }
}

pub fn cast_bar_update(
    time: Res<Time>,
    mut cast_bar_query: Query<(&Parent, &mut Style), With<CastBarMarker>>,
    cast_query: Query<&CastDuration, With<CastMarker>>,
    nameplate_query: Query<&NameplateParent, With<NameplateMarker>>,
    character_query: Query<&CastState, With<CharacterMarker>>,
) {
    let now = time.last_update().expect("last update not found");

    for (cast_bar_parent, mut cast_bar_style) in cast_bar_query.iter_mut() {
        let character_cast = nameplate_query
            .get(cast_bar_parent.0)
            .and_then(|nameplate_parent| character_query.get(nameplate_parent.0))
            .map(|character_cast_state| character_cast_state.0.as_ref());

        match character_cast {
            Ok(Some(Cast { start, cast_id })) => {
                cast_bar_style.display = Display::Flex;

                let total_cast_duration = cast_query
                    .get(*cast_id)
                    .expect("casts must have a cast duration");

                let current_duration = now - *start;
                let percent =
                    100. * current_duration.as_secs_f32() / total_cast_duration.0.as_secs_f32();

                cast_bar_style.size.width = Val::Percent(percent);
            }
            Ok(None) => {
                cast_bar_style.display = Display::None;
            }
            _ => (),
        }
    }
}
