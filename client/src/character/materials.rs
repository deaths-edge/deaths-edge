use bevy::prelude::*;

use common::character::CharacterClass;

pub struct CharacterMaterials {
    medea_material: Handle<ColorMaterial>,
    mars_material: Handle<ColorMaterial>,
}

impl FromWorld for CharacterMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        Self {
            medea_material: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            mars_material: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
        }
    }
}

impl CharacterMaterials {
    pub fn handle(&self, class: CharacterClass) -> &Handle<ColorMaterial> {
        use CharacterClass::*;
        match class {
            Medea => &self.medea_material,
            Mars => &self.mars_material,
            _ => todo!(),
        }
    }
}

pub struct CharacterMaterialPlugin;

impl Plugin for CharacterMaterialPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<CharacterMaterials>();
    }
}
