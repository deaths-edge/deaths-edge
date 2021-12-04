mod health;
mod materials;
mod parent;
mod power;

use std::ops::Deref;

use bevy::prelude::*;

pub use health::*;
pub use materials::*;
pub use parent::*;
pub use power::*;

use crate::{character::CharacterMarker, ui::mouse::local_to_window_position};

use super::PlayerCamera;

pub struct NameplatePlugin;

impl Plugin for NameplatePlugin {
    fn build(&self, app: &mut AppBuilder) {
        let nameplate_system_set = SystemSet::new()
            .with_system(update_nameplate_position.system())
            .with_system(health_bar_update.system());
        app.init_resource::<NameplateMaterials>()
            .add_system_set(nameplate_system_set);
    }
}

#[derive(Debug)]
pub struct NameplateMarker;

#[derive(Debug)]
pub struct NameplateOffset(Size<Val>);

impl Deref for NameplateOffset {
    type Target = Size<Val>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Size<Val>> for NameplateOffset {
    fn from(value: Size<Val>) -> Self {
        Self(value)
    }
}

#[derive(Debug, Bundle)]
pub struct NameplateBundle {
    marker: NameplateMarker,
    parent: NameplateParent,
    offset: NameplateOffset,
    #[bundle]
    node: NodeBundle,
}

impl NameplateBundle {
    pub fn new(parent: NameplateParent, nameplate_materials: &NameplateMaterials) -> Self {
        let width_pct = 8.;
        let height_pct = 2.5;
        let size = Size::new(Val::Percent(width_pct), Val::Percent(height_pct));
        let offset = NameplateOffset::from(Size::new(
            Val::Percent(width_pct / 2.),
            Val::Percent(height_pct * 2.),
        ));

        Self {
            marker: NameplateMarker,
            parent,
            offset,
            node: NodeBundle {
                style: Style {
                    size,
                    position_type: PositionType::Absolute,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    ..Default::default()
                },
                material: nameplate_materials.none.clone(),
                ..Default::default()
            },
        }
    }
}

pub fn setup_nameplate(
    character_entity: In<Entity>,

    nameplate_materials: Res<NameplateMaterials>,

    mut commands: Commands,
) {
    let nameplate_bundle = NameplateBundle::new(character_entity.0.into(), &nameplate_materials);
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
    windows: Res<Windows>,

    mut nameplate_query: Query<
        (&NameplateParent, &NameplateOffset, &mut Style),
        With<NameplateMarker>,
    >,

    character_query: Query<&Transform, (With<CharacterMarker>, Changed<Transform>)>,

    camera_query: Query<&Transform, With<PlayerCamera>>,
) {
    let camera_transform = camera_query
        .single()
        .expect("there must be a player camera");

    for (nameplate_parent, node_offset, mut node_style) in nameplate_query.iter_mut() {
        let character_transform = character_query
            .get(nameplate_parent.id())
            .expect("character not found");

        let primary_window = windows.get_primary().expect("no monitor");

        let window_position = local_to_window_position(
            primary_window,
            camera_transform,
            character_transform.translation,
        );

        let offset_width_px = match node_offset.width {
            Val::Px(px) => px,
            Val::Percent(pct) => pct / 100. * primary_window.width(),
            Val::Undefined => 0.,
            Val::Auto => 0.,
        };
        let offset_height_px = match node_offset.height {
            Val::Px(px) => px,
            Val::Percent(pct) => pct / 100. * primary_window.height(),
            Val::Undefined => 0.,
            Val::Auto => 0.,
        };

        node_style.position.left = Val::Px(window_position.x) + -offset_width_px;
        node_style.position.bottom = Val::Px(window_position.y) + offset_height_px;
    }
}
