mod health;
mod materials;
mod parent;
mod power;

use bevy::prelude::*;

pub use health::*;
pub use materials::*;
pub use parent::*;
pub use power::*;

use crate::character::{CharacterIndex, CharacterMarker};

pub struct NameplatePlugin;

impl Plugin for NameplatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<NameplateMaterials>()
            .add_system(update_nameplate_position.system());
    }
}

#[derive(Debug)]
pub struct NameplateMarker;

#[derive(Debug, Bundle)]
pub struct NameplateBundle {
    marker: NameplateMarker,
    parent: NameplateParent,
    #[bundle]
    node: NodeBundle,
}

impl NameplateBundle {
    pub fn new(parent: NameplateParent, nameplate_materials: &NameplateMaterials) -> Self {
        Self {
            marker: NameplateMarker,
            parent,
            node: NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(7.5), Val::Percent(2.5)),
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::FlexEnd,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    position: Rect {
                        left: Val::Px(400.0),
                        bottom: Val::Px(300.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                material: nameplate_materials.primary_health_bar.clone(),
                ..Default::default()
            },
        }
    }
}

pub fn setup_nameplate(
    character_index: In<CharacterIndex>,

    nameplate_materials: Res<NameplateMaterials>,

    mut commands: Commands,
) {
    let nameplate_bundle = NameplateBundle::new(character_index.0.into(), &nameplate_materials);
    commands
        .spawn_bundle(nameplate_bundle)
        .with_children(|commands| {
            let health_bar_bundle = HealthBarBundle::new(&nameplate_materials);
            let power_bar_bundle = PowerBarBundle::new(&nameplate_materials);

            commands.spawn_bundle(health_bar_bundle);
            commands.spawn_bundle(power_bar_bundle);
        });
}

pub fn update_nameplate_position(
    mut nameplate_query: Query<(&NameplateParent, &mut NodeBundle), With<NameplateMarker>>,
    character_query: Query<
        (&CharacterIndex, &Transform),
        (With<CharacterMarker>, Changed<Transform>),
    >,
) {
    for (nameplate_parent, mut node_bundle) in nameplate_query.iter_mut() {
        let (_, character_transform) = character_query
            .iter()
            .find(|(index, _)| nameplate_parent == *index)
            .expect("character not found");
        tracing::info!("found char");
        node_bundle.transform = *character_transform;
    }
}
