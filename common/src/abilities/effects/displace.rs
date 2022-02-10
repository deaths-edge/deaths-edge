use bevy::{math::Vec2, prelude::*};

use super::CharacterEffect;

#[derive(Default, Debug, Clone, Component)]
pub struct Displace(pub Vec2);

impl CharacterEffect for Displace {
    type Domain<'a> = &'a mut Transform;

    type Param<'w, 's> = ();
    type Fetch = ();

    fn apply(
        &self,
        _parent_id: Entity,

        mut item: Mut<'_, Transform>,
        _param: &(),

        _time: &Time,

        _commands: &mut Commands,
    ) {
        let offset = item.rotation * self.0.extend(0.);
        item.translation += offset;
    }
}
