mod fireball;
mod spear;

use bevy::prelude::*;

pub use fireball::*;
pub use spear::*;

pub trait ToEffect: Bundle + Sized {
    type Effect: From<Self> + Bundle;

    fn process_spell(id: Entity, world: &mut World) {
        let mut spell_entity_mut = world.entity_mut(id);

        let bundle = spell_entity_mut
            .remove_bundle::<Self>()
            .expect("fireball bundle not found");

        let effect: Self::Effect = bundle.into();
        world.spawn().insert_bundle(effect);
    }
}
