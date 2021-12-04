mod cast;
mod impact;
mod index;
pub mod instances;
mod marker;
mod source;
mod target;

use bevy::prelude::*;

use crate::{character::CharacterIndex, physics::Velocity};

pub use cast::*;
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
            .add_system(spell_impact_system.exclusive_system());
    }
}

pub struct SpellProjectileMarker;

pub fn spell_tracking(
    mut spell_query: Query<
        (&SpellTarget, &mut Transform, &mut Velocity),
        (With<SpellProjectileMarker>, With<SpellMarker>),
    >,
    char_query: Query<(Entity, &Transform), Without<SpellMarker>>,
) {
    for (spell_target, mut spell_transform, mut spell_velocity) in spell_query.iter_mut() {
        if let Ok((_, char_transform)) = char_query.get(spell_target.id()) {
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
