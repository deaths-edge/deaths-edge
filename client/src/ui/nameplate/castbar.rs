use bevy::prelude::*;

use common::{
    abilities::lifecycle::{CastMarker, ProgressDuration, TotalDuration},
    character::{CastId, CharacterMarker},
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
    mut cast_bar_query: Query<(&Parent, &mut Style), With<CastBarMarker>>,
    cast_query: Query<(&ProgressDuration, &TotalDuration), With<CastMarker>>,
    nameplate_query: Query<&NameplateParent, With<NameplateMarker>>,
    character_query: Query<&CastId, With<CharacterMarker>>,
) {
    for (cast_bar_parent, mut cast_bar_style) in cast_bar_query.iter_mut() {
        let cast_id_res = nameplate_query
            .get(cast_bar_parent.0)
            .and_then(|nameplate_parent| character_query.get(nameplate_parent.0));

        match cast_id_res {
            Ok(&CastId(cast_id)) => {
                info!("update castbar");
                cast_bar_style.display = Display::Flex;

                let (progress_cast_duration, total_cast_duration) = cast_query
                    .get(cast_id)
                    .expect("casts must have a cast duration");

                let percent = 100. * progress_cast_duration.0.as_secs_f32()
                    / total_cast_duration.0.as_secs_f32();

                cast_bar_style.size.width = Val::Percent(percent);
            }
            _ => {
                cast_bar_style.display = Display::None;
            }
        }
    }
}
