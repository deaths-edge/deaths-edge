use bevy::prelude::*;
use common::{
    abilities::{
        obstructions::{Cooldown, GlobalCooldown},
        AbilityId, AbilityMarker,
    },
    character::{Abilities, LastCastInstant, GLOBAL_COOLDOWN},
};

use crate::character::{PlayerMarker, PlayerState};

#[derive(Debug, Default, Component)]
pub struct ButtonRowMarker;

#[derive(Bundle)]
pub struct ButtonRow {
    marker: ButtonRowMarker,
    #[bundle]
    node: NodeBundle,
}

#[derive(Debug, Default, Component)]
pub struct ButtonMarker;

#[derive(Bundle)]
pub struct Button {
    marker: ButtonMarker,

    ability_id: AbilityId,

    #[bundle]
    node: NodeBundle,
}

#[derive(Debug, Default, Component)]
pub struct ButtonCooldownMarker;

#[derive(Debug, Default, Bundle)]
pub struct ButtonCooldown {
    marker: ButtonCooldownMarker,

    #[bundle]
    node: NodeBundle,
}

fn spawn_buttons(player_query: Query<&Abilities, With<PlayerMarker>>, mut commands: Commands) {
    let Abilities(abilities) = player_query.single();

    let button_row = ButtonRow {
        marker: ButtonRowMarker,
        node: NodeBundle {
            color: Color::NONE.into(),
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(5.0)),
                flex_direction: FlexDirection::Row,
                align_content: AlignContent::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            ..Default::default()
        },
    };
    commands.spawn_bundle(button_row).with_children(|commands| {
        for ability_id in abilities.iter().cloned() {
            let button = Button {
                marker: ButtonMarker,

                ability_id,

                node: NodeBundle {
                    style: Style {
                        size: Size::new(
                            Val::Percent(100.0 / abilities.len() as f32),
                            Val::Percent(100.0),
                        ),
                        aspect_ratio: Some(1.0),
                        margin: Rect {
                            left: Val::Percent(1.0),
                            right: Val::Percent(1.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    color: Color::OLIVE.into(),
                    ..Default::default()
                },
            };
            commands.spawn_bundle(button).with_children(|commands| {
                let button_cooldown = ButtonCooldown {
                    marker: ButtonCooldownMarker,

                    node: NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Percent(0.0)),
                            ..Default::default()
                        },
                        color: Color::GRAY.into(),
                        ..Default::default()
                    },
                };
                commands.spawn_bundle(button_cooldown);
            });
        }
    });
}

fn update_button_cooldown(
    time: Res<Time>,

    mut cooldown_query: Query<(&Parent, &mut Style), With<ButtonCooldownMarker>>,
    button_query: Query<&AbilityId, With<ButtonMarker>>,
    requires_query: Query<&GlobalCooldown, With<AbilityMarker>>,
    ability_query: Query<(&LastCastInstant, &Cooldown), With<AbilityMarker>>,
    player_query: Query<&LastCastInstant, With<PlayerMarker>>,
) {
    let now = time.last_update().expect("failed to find last time update");
    for (parent, mut style) in cooldown_query.iter_mut() {
        let AbilityId(ability_id) = button_query
            .get(parent.0)
            .expect("failed to find parent button");

        let character_last_cast_instant = player_query.single();

        let opt_global_cooldown = if requires_query.get(*ability_id).is_ok() {
            let finish = character_last_cast_instant.0 + GLOBAL_COOLDOWN;
            let remaining = finish.saturating_duration_since(now);
            let ratio = remaining.as_secs_f32() / GLOBAL_COOLDOWN.as_secs_f32();

            Some((remaining, ratio))
        } else {
            None
        };

        let opt_cooldown =
            if let Ok((ability_last_cast_instant, cooldown)) = ability_query.get(*ability_id) {
                let finish = ability_last_cast_instant.0 + cooldown.0;
                let remaining = finish.saturating_duration_since(now);

                let ratio = remaining.as_secs_f32() / GLOBAL_COOLDOWN.as_secs_f32();

                Some((remaining, ratio))
            } else {
                None
            };

        let ratio = match (opt_global_cooldown, opt_cooldown) {
            (None, None) => 0.0,
            (None, Some((_, ratio))) => ratio,
            (Some((_, ratio)), None) => ratio,
            (Some((global_remaining, global_ratio)), Some((remaining, ratio))) => {
                if global_remaining < remaining {
                    ratio
                } else {
                    global_ratio
                }
            }
        };

        style.size.height = Val::Percent(ratio * 100.0);
    }
}

pub struct ButtonsPlugin;

impl Plugin for ButtonsPlugin {
    fn build(&self, app: &mut App) {
        let buttons_setup = SystemSet::on_enter(PlayerState::Spawned).with_system(spawn_buttons);

        let cooldown_update =
            SystemSet::on_update(PlayerState::Spawned).with_system(update_button_cooldown);
        app.add_system_set(buttons_setup)
            .add_system_set(cooldown_update);
    }
}
