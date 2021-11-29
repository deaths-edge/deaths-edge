use bevy::prelude::*;

pub struct NameplateMaterials {
    pub none: Handle<ColorMaterial>,
    pub primary_health_bar: Handle<ColorMaterial>,
    pub energy_bar: Handle<ColorMaterial>,
}

impl FromWorld for NameplateMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        NameplateMaterials {
            none: materials.add(Color::NONE.into()),
            primary_health_bar: materials.add(Color::TOMATO.into()),
            energy_bar: materials.add(Color::SILVER.into()),
        }
    }
}
