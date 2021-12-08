use bevy::prelude::*;

pub struct NameplateMaterials {
    pub none: Handle<ColorMaterial>,
    pub health_bar: Handle<ColorMaterial>,
    pub energy_bar: Handle<ColorMaterial>,
    pub cast_bar: Handle<ColorMaterial>,
}

impl FromWorld for NameplateMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        Self {
            none: materials.add(Color::rgba(0., 0., 0., 0.5).into()),
            health_bar: materials.add(Color::GREEN.into()),
            energy_bar: materials.add(Color::BLUE.into()),
            cast_bar: materials.add(Color::YELLOW.into()),
        }
    }
}
