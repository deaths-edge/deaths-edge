use bevy::prelude::*;

use crate::character::PlayerMarker;

use common::{abilities::Target, character::CharacterMarker};

use super::HudState;

#[derive(Debug, Default, Component)]
pub struct SelectionMarker;

#[derive(Component, Debug, Default)]
pub struct OptionalTarget(pub Option<Target>);

#[derive(Bundle)]
pub struct SelectionBundle {
    marker: SelectionMarker,
    parent: OptionalTarget,
    #[bundle]
    sprite: SpriteBundle,
}

fn spawn_selection(mut commands: Commands) {
    let bundle = SelectionBundle {
        marker: SelectionMarker,
        parent: OptionalTarget(None),
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::YELLOW.into(),
                ..Default::default()
            },
            ..Default::default()
        },
    };
    commands.spawn_bundle(bundle);
    info!("spawned selection");
}

fn select_changed(
    player_query: Query<Option<&Target>, With<PlayerMarker>>,
    char_query: Query<&Sprite, (With<CharacterMarker>, Without<SelectionMarker>)>,
    mut select_query: Query<
        (&mut OptionalTarget, &mut Visibility, &mut Sprite),
        (With<SelectionMarker>, Without<PlayerMarker>),
    >,
) {
    if let Ok(player_target) = player_query.get_single() {
        let (mut select_target, mut visibility, mut select_sprite) = select_query.single_mut();
        select_target.0 = player_target.cloned();
        if let Some(target_id) = player_target {
            let target_sprite = char_query.get(target_id.0).expect("failed to find target");
            select_sprite.custom_size = target_sprite.custom_size.map(|x| x * 1.2);
        }

        visibility.is_visible = player_target.is_some();
    }
}

fn select_follow(
    mut selected_query: Query<
        (&mut Transform, &OptionalTarget),
        (With<SelectionMarker>, Without<CharacterMarker>),
    >,
    character_query: Query<&Transform, With<CharacterMarker>>,
) {
    let (mut selected_transform, opt_target) = selected_query.single_mut();

    if let Some(target) = opt_target.0 {
        let transform = character_query
            .get(target.0)
            .expect("failed to find selection");
        *selected_transform = *transform;
    }
}

pub struct SelectedPlugin;

impl Plugin for SelectedPlugin {
    fn build(&self, app: &mut App) {
        let selection = SystemSet::on_update(HudState::Active)
            .with_system(select_changed)
            .with_system(select_follow);
        let selection_spawn = SystemSet::on_enter(HudState::Active).with_system(spawn_selection);
        app.add_system_set(selection_spawn)
            .add_system_set(selection);
    }
}
