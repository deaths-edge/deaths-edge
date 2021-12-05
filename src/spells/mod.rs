mod cast;
mod impact;
mod index;
pub mod instances;
mod marker;
mod source;
mod target;

use bevy::prelude::*;
use heron::prelude::*;

pub use cast::*;
pub use impact::*;
pub use marker::*;
pub use source::*;
pub use target::*;

use instances::SpellMaterials;

use crate::character::CharacterMarker;

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SpellMaterials>()
            .add_event::<SpellImpactEvent>()
            .add_system(spell_tracking.system())
            .add_system(spell_projectile_collision.system())
            .add_system(spell_impact_system.exclusive_system());
    }
}

pub struct SpellProjectileMarker;

pub fn spell_tracking(
    mut spell_query: Query<
        (&SpellTarget, &mut Transform, &mut Velocity),
        (With<SpellProjectileMarker>, With<SpellMarker>),
    >,
    character_query: Query<(Entity, &Transform), Without<SpellMarker>>,
) {
    for (spell_target, mut spell_transform, mut spell_velocity) in spell_query.iter_mut() {
        if let Ok((_, char_transform)) = character_query.get(spell_target.id()) {
            let diff = (char_transform.translation - spell_transform.translation).truncate();
            let angle = Vec2::new(1., 0.).angle_between(diff);
            spell_transform.rotation = Quat::from_rotation_z(angle);

            *spell_velocity = spell_transform
                .rotation
                .mul_vec3(Vec3::new(spell_velocity.linear.length(), 0., 0.))
                .into();
        }
    }
}

pub fn spell_projectile_collision(
    mut contact_events: EventReader<CollisionEvent>,
    mut spell_impact_events: EventWriter<SpellImpactEvent>,

    spell_query: Query<
        (Entity, &SpellMarker, &SpellTarget),
        (With<SpellProjectileMarker>, With<SpellMarker>),
    >,
    character_query: Query<Entity, (With<CharacterMarker>, Without<SpellMarker>)>,
) {
    for collision_event in contact_events.iter() {
        if let CollisionEvent::Started(collision_data_1, collision_data_2) = collision_event {
            for (spell_entity, spell_marker, spell_target) in spell_query.iter() {
                if let Ok(character_entity) = character_query.get(spell_target.id()) {
                    let eq_a = spell_entity == collision_data_1.rigid_body_entity()
                        && character_entity == collision_data_2.rigid_body_entity();
                    let eq_b = spell_entity == collision_data_2.rigid_body_entity()
                        && character_entity == collision_data_1.rigid_body_entity();

                    if eq_a || eq_b {
                        let impact_event = SpellImpactEvent {
                            id: spell_entity,
                            spell_marker: *spell_marker,
                        };
                        spell_impact_events.send(impact_event);
                    }
                }
            }
        }
    }
}
