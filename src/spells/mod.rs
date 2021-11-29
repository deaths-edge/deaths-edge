mod cast;
pub mod effects;
mod impact;
mod index;
pub mod instances;
mod marker;
mod source;
mod target;

use bevy::prelude::*;

use crate::{character::CharacterIndex, physics::Velocity};

pub use cast::*;
pub use effects::*;
pub use impact::*;
pub use marker::*;
pub use source::*;
pub use target::*;

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let spell_tracking = SystemSet::new().with_system(spell_tracking.system());
        app.add_event::<SpellImpactEvent>()
            .add_system_set(spell_tracking)
            .add_system(spell_impact_system.exclusive_system())
            .add_system(damage_effect_apply.system());
    }
}

pub struct SpellProjectileMarker;

pub fn spell_tracking(
    mut spell_query: Query<
        (&mut Transform, &mut Velocity, &SpellTarget),
        (With<SpellProjectileMarker>, With<SpellMarker>),
    >,
    char_query: Query<(&Transform, &CharacterIndex), Without<SpellMarker>>,
) {
    for (mut spell_transform, mut spell_velocity, spell_target) in spell_query.iter_mut() {
        let char_target = char_query.iter().find(|(_, index)| spell_target == *index);
        if let Some((char_transform, _)) = char_target {
            let diff = (char_transform.translation - spell_transform.translation).truncate();
            let angle = Vec2::new(1., 0.).angle_between(diff);
            spell_transform.rotation = Quat::from_rotation_z(angle);

            **spell_velocity = spell_transform
                .rotation
                .mul_vec3(Vec3::new(spell_velocity.length(), 0., 0.))
                .truncate();
        }
    }
}
