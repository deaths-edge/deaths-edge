use bevy::prelude::*;

use super::CharacterClass;

pub struct CharacterMaterials {
    medea_material: Handle<ColorMaterial>,
    mars_material: Handle<ColorMaterial>,
    pluto_material: Handle<ColorMaterial>,
    mammon_material: Handle<ColorMaterial>,
    heka_material: Handle<ColorMaterial>,
}

impl FromWorld for CharacterMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        CharacterMaterials {
            medea_material: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            mars_material: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            pluto_material: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            mammon_material: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            heka_material: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
        }
    }
}

impl CharacterMaterials {
    pub fn handle(&self, class: CharacterClass) -> &Handle<ColorMaterial> {
        use CharacterClass::*;
        match class {
            Medea => &self.medea_material,
            Mars => &self.mars_material,
            Pluto => &self.pluto_material,
            Mammon => &self.mammon_material,
            Heka => &self.heka_material,
        }
    }
}

pub struct CharacterMaterialPlugins;

impl Plugin for CharacterMaterialPlugins {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<CharacterMaterials>();
    }
}
