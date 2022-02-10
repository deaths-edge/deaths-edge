use std::time::Duration;

use bevy::prelude::*;
use common::{
    abilities::{
        magic_school::{Fire, Frost, Nature},
        obstructions::{OnCooldown, OnGlobalCooldown},
        AbilityId, AbilityMarker,
    },
    character::{Abilities, Interrupted, LastCastInstant, GLOBAL_COOLDOWN},
};

use crate::{
    character::{PlayerMarker, PlayerState},
    ui::hud::HudState,
};

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

#[derive(Component)]
pub struct ButtonIndex(pub usize);

#[derive(Bundle)]
pub struct Button {
    marker: ButtonMarker,

    button_index: ButtonIndex,

    #[bundle]
    node: NodeBundle,
}

#[derive(Debug, Default, Component)]
pub struct ButtonCooldownMarker;

#[derive(Debug, Default, Component)]
struct CooldownRemainder {
    duration: Duration,
    ratio: f32,
}

impl CooldownRemainder {
    fn displace(&mut self, other: CooldownRemainder) {
        if self.duration < other.duration {
            *self = other;
        }
    }
}

#[derive(Debug, Default, Bundle)]
pub struct ButtonCooldown {
    marker: ButtonCooldownMarker,

    #[bundle]
    node: NodeBundle,

    cooldown_remainder: CooldownRemainder,
}

fn attach_abilities(
    player_query: Query<&Abilities, Added<PlayerMarker>>,
    button_query: Query<(Entity, &ButtonIndex)>,

    mut commands: Commands,
) {
    info!("attaching");
    if let Ok(Abilities(abilities)) = player_query.get_single() {
        for (id, btn_index) in button_query.iter() {
            commands.entity(id).insert(abilities[btn_index.0]);
        }
    }
}

fn spawn_buttons(mut commands: Commands) {
    const N_ABILITIES: usize = 8;

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
        for index in 0..N_ABILITIES {
            let button = Button {
                marker: ButtonMarker,

                button_index: ButtonIndex(index),

                node: NodeBundle {
                    style: Style {
                        size: Size::new(
                            Val::Percent(100.0 / N_ABILITIES as f32),
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

                    cooldown_remainder: CooldownRemainder {
                        duration: Duration::from_secs(0),
                        ratio: 0.0,
                    },
                };
                commands.spawn_bundle(button_cooldown);
            });
        }
    });
}

fn update_global_cooldown_remainder(
    time: Res<Time>,
    mut cooldown_query: Query<(&Parent, &mut CooldownRemainder), With<ButtonCooldownMarker>>,
    button_query: Query<&AbilityId, With<ButtonMarker>>,

    requires_query: Query<(), (With<AbilityMarker>, With<OnGlobalCooldown>)>,

    player_query: Query<&LastCastInstant, With<PlayerMarker>>,
) {
    let now = time.last_update().expect("failed to find last time update");
    for (parent, mut remainder) in cooldown_query.iter_mut() {
        let AbilityId(ability_id) = if let Ok(&ok) = button_query.get(parent.0) {
            ok
        } else {
            continue;
        };

        if requires_query.get(ability_id).is_ok() {
            let character_last_cast_instant = player_query.single();

            if let Some(last_cast) = character_last_cast_instant.0 {
                let finish = last_cast + GLOBAL_COOLDOWN;
                let duration = finish.saturating_duration_since(now);
                let ratio = duration.as_secs_f32() / GLOBAL_COOLDOWN.as_secs_f32();

                let new_remainder = CooldownRemainder { duration, ratio };
                remainder.displace(new_remainder)
            }
        }
    }
}

fn update_interrupt_remainder<School: Component>(
    time: Res<Time>,
    mut cooldown_query: Query<(&Parent, &mut CooldownRemainder), With<ButtonCooldownMarker>>,
    button_query: Query<&AbilityId, With<ButtonMarker>>,

    requires_query: Query<&School, With<AbilityMarker>>,

    player_query: Query<&Interrupted<School>, With<PlayerMarker>>,
) {
    let now = time.last_update().expect("failed to find last time update");
    for (parent, mut remainder) in cooldown_query.iter_mut() {
        let AbilityId(ability_id) = if let Ok(&ok) = button_query.get(parent.0) {
            ok
        } else {
            continue;
        };

        if requires_query.get(ability_id).is_ok() {
            if let Ok(&Interrupted { until, start, .. }) = player_query.get_single() {
                let duration = until.saturating_duration_since(now);
                let total = until - start;
                let ratio = duration.as_secs_f32() / total.as_secs_f32();

                let new_remainder = CooldownRemainder { duration, ratio };
                remainder.displace(new_remainder)
            }
        }
    }
}

fn update_cooldown_remainder(
    time: Res<Time>,
    mut cooldown_query: Query<(&Parent, &mut CooldownRemainder), With<ButtonCooldownMarker>>,
    button_query: Query<&AbilityId, With<ButtonMarker>>,

    ability_query: Query<(&LastCastInstant, &OnCooldown), With<AbilityMarker>>,
) {
    let now = time.last_update().expect("failed to find last time update");
    for (parent, mut remainder) in cooldown_query.iter_mut() {
        let AbilityId(ability_id) = if let Ok(&ok) = button_query.get(parent.0) {
            ok
        } else {
            continue;
        };

        if let Ok((last_cast_instant, cooldown)) = ability_query.get(ability_id) {
            if let Some(last_cast) = last_cast_instant.0 {
                let finish = last_cast + cooldown.0;
                let duration = finish.saturating_duration_since(now);
                let ratio = duration.as_secs_f32() / cooldown.0.as_secs_f32();

                let new_remainder = CooldownRemainder { duration, ratio };
                remainder.displace(new_remainder)
            }
        }
    }
}

fn update_button_cooldown(
    mut cooldown_query: Query<(&mut Style, &mut CooldownRemainder), With<ButtonCooldownMarker>>,
) {
    for (mut style, mut remainder) in cooldown_query.iter_mut() {
        style.size.height = Val::Percent(remainder.ratio * 100.0);

        // Reset remainder duration
        remainder.duration = Duration::from_secs(0);
    }
}

pub struct ButtonsPlugin;

impl Plugin for ButtonsPlugin {
    fn build(&self, app: &mut App) {
        // TODO: Make this HUD state
        let buttons_setup = SystemSet::on_enter(HudState::Active).with_system(spawn_buttons);

        // TODO: Improve lifecycle here
        let attach_abilities =
            SystemSet::on_update(PlayerState::Spawned).with_system(attach_abilities);

        const COOLDOWN_REMAINDER_UPDATES: &str = "cooldown-remainder-updates";
        let cooldown_updates = SystemSet::on_update(PlayerState::Spawned)
            .label(COOLDOWN_REMAINDER_UPDATES)
            .with_system(update_global_cooldown_remainder)
            .with_system(update_cooldown_remainder)
            .with_system(update_interrupt_remainder::<Fire>)
            .with_system(update_interrupt_remainder::<Frost>)
            .with_system(update_interrupt_remainder::<Nature>);

        let cooldown_button_update = SystemSet::on_update(PlayerState::Spawned)
            .after(COOLDOWN_REMAINDER_UPDATES)
            .with_system(update_button_cooldown);
        app.add_system_set(buttons_setup)
            .add_system_set(attach_abilities)
            .add_system_set(cooldown_updates)
            .add_system_set(cooldown_button_update);
    }
}
