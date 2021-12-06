use bevy::prelude::*;

pub struct SpellMaterials {
    pub fireball_material: Handle<ColorMaterial>,
}
impl FromWorld for SpellMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        Self {
            fireball_material: materials.add(Color::RED.into()),
        }
    }
}
