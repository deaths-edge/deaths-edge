use std::{fmt::Debug, hash::Hash};

use bevy::{core::Time, prelude::*, utils::Instant};
use heron::rapier_plugin::PhysicsWorld;

use super::CharacterMarker;
use crate::spells::{check_in_front, check_line_of_sight, Spell};

#[derive(Default, Debug)]
pub struct CharacterCastState {
    cast: Option<CharacterCast>,
}

impl CharacterCastState {
    pub fn set_cast(&mut self, cast: CharacterCast) -> &mut Self {
        self.cast = Some(cast);
        self
    }

    pub fn stop_cast(&mut self) -> Option<CharacterCast> {
        self.cast.take()
    }

    pub fn cast(&self) -> Option<&CharacterCast> {
        self.cast.as_ref()
    }
}

#[derive(Debug)]
pub struct CharacterCast {
    pub start: Instant,
    pub spell: Spell,
}

impl CharacterCast {
    pub fn new(start: Instant, spell: Spell) -> Self {
        Self { start, spell }
    }

    pub fn is_complete(&self, now: Instant) -> bool {
        self.start + self.spell.duration() <= now
    }
}

fn complete_casting(
    mut cast_query: Query<(&Transform, &mut CharacterCastState)>,
    target_query: Query<&Transform, With<CharacterMarker>>,

    physics_world: PhysicsWorld,

    time: Res<Time>,

    mut spell_writer: EventWriter<Spell>,
) {
    let last_update = time.last_update().expect("last update not found");

    for (character_transform, mut cast_state) in cast_query.iter_mut().filter(|(_, cast_state)| {
        cast_state
            .cast()
            .map(|cast| cast.is_complete(last_update))
            .unwrap_or_default()
    }) {
        tracing::info!(message = "cast completed", ?cast_state);
        let cast = cast_state.stop_cast().expect("checked valid");

        // Check targeting rules
        if let Some(targeting) = cast.spell.targeting() {
            let target_transform = target_query
                .get(targeting.target.0)
                .expect("failed to find target");

            let is_los = check_line_of_sight(
                character_transform,
                target_transform.translation,
                &physics_world,
            )
            .is_ok();
            if !is_los {
                warn!("failed line-of-sight check");
                continue;
            }

            if targeting.requires_fov {
                let is_in_front =
                    check_in_front(character_transform, target_transform.translation).is_ok();

                if !is_in_front {
                    warn!("failed fov check");
                    continue;
                }
            }
        }

        // TODO: Send batch
        spell_writer.send(cast.spell);
    }
}

pub struct CastingPlugin<T> {
    state: T,
}

impl<T> CastingPlugin<T> {
    pub fn new(state: T) -> Self {
        Self { state }
    }
}

pub const CASTING_LABEL: &str = "casting";

impl<T> Plugin for CastingPlugin<T>
where
    T: Sync + Send + Debug + Clone + Copy + Eq + Hash + 'static,
{
    fn build(&self, app: &mut AppBuilder) {
        let casting_system = SystemSet::on_update(self.state)
            .label(CASTING_LABEL)
            .with_system(complete_casting.system());
        app.add_event::<Spell>().add_system_set(casting_system);
    }
}
