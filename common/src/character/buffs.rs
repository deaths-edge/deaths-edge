use std::time::Duration;

use bevy::{prelude::*, utils::Instant};

use crate::{character::Control, effects::ControlEffect};

use super::CharacterMarker;

#[derive(Debug, Default)]
pub struct Buffs(pub Vec<Buff>);

#[derive(Debug, Clone, Copy)]
pub enum Buff {
    Speared { start: Instant },
}

pub const SPEARED_DURATION: Duration = Duration::from_secs(3);

pub fn buff_lifecycle(
    time: Res<Time>,

    buff_query: Query<(Entity, &Buffs), With<CharacterMarker>>,

    mut commands: Commands,
) {
    for (host, buffs) in buff_query.iter() {
        for buff in &buffs.0 {
            use Buff::*;
            match buff {
                Speared { start } => {
                    let last_update = time.last_update().expect("last update not found");
                    let elapsed = last_update - *start;
                    if elapsed > SPEARED_DURATION {
                        commands
                            .spawn()
                            .insert(ControlEffect::remove(Control::Root));
                    }
                }
            }
        }
    }
}
