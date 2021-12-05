use bevy::prelude::*;
use heron::rapier_plugin::PhysicsWorld;

use super::*;
use crate::{
    effects::{EffectMarker, EffectTarget, InteruptEffect},
    input_mapping::ActionKey,
    spells::{
        check_in_front, check_line_of_sight, instances::fireball_action, SpellCast, SpellSource,
    },
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let character_motion = SystemSet::new()
            .label("character-motion")
            .before("collisions")
            .with_system(player_focal_rotate.system())
            .with_system(player_movement.system());
        let character_actions = SystemSet::new()
            .label("character-actions")
            .with_system(player_action.system())
            .with_system(player_char_select.system());
        app.add_system_set(character_motion)
            .add_system_set(character_actions);
    }
}

pub struct PlayerMarker;

#[derive(Bundle)]
pub struct PlayerBundle {
    player_marker: PlayerMarker,
    #[bundle]
    character_bundle: CharacterBundle,
}

impl PlayerBundle {
    pub fn new(
        index: CharacterIndex,
        class: CharacterClass,
        transform: Transform,
        time: &Time,
        materials: &CharacterMaterials,
    ) -> Self {
        Self {
            player_marker: PlayerMarker,
            character_bundle: CharacterBundle::new(index, class, transform, time, materials),
        }
    }
}

/// Receives a [`SelectClick`] event and selects a character.
pub fn player_char_select(
    mut select_clicks: EventReader<SelectClick>,
    mut character_query: QuerySet<(
        Query<(Entity, &mut Selected)>,
        Query<(Entity, &Transform, &Sprite, &mut Selected)>,
        Query<&mut CharacterTarget, With<PlayerMarker>>,
    )>,
) {
    const SELECT_SIZE: (f32, f32) = (30., 30.);

    // Get selected click
    let select_click = if let Some(some) = select_clicks.iter().last() {
        some
    } else {
        return;
    };

    // Find and set selection
    let selected_entity_opt = character_query
        .q1_mut()
        .iter_mut()
        .find(|(_, char_transform, char_sprite, _)| {
            collide(
                select_click.mouse_position.extend(0.),
                SELECT_SIZE.into(),
                char_transform.translation,
                char_sprite.size,
            )
            .is_some()
        })
        .map(|(entity, _, _, selected)| (entity, selected))
        .map(|(entity, mut selected)| {
            // Set selection
            *selected = Selected::Selected;

            entity
        });

    // Set character selection
    if let Ok(mut character_target) = character_query.q2_mut().single_mut() {
        if let Some(index) = selected_entity_opt {
            tracing::info!(message = "selected character", ?index);
            character_target.set_entity(index);
        } else {
            character_target.deselect();
        }
    };

    // Deselect everything else
    for (_, mut selected) in character_query
        .q0_mut()
        .iter_mut()
        .filter(|(entity, _)| Some(*entity) != selected_entity_opt)
    {
        *selected = Selected::Unselected;
    }
}

/// Receives [`FocalHold`] event and rotates character in that direction.
pub fn player_focal_rotate(
    mut character_query: Query<&mut Transform, With<PlayerMarker>>,
    mut events: EventReader<FocalHold>,
) {
    let mut transform = character_query.single_mut().expect("player not found");

    const MINIMUM_FOCAL_LENGTH: f32 = 200.;

    if let Some(event) = events.iter().last() {
        let translation = Vec2::from(transform.translation);

        let diff = event.mouse_position - translation;
        let distance = diff.length();
        let adjustment = distance.min(MINIMUM_FOCAL_LENGTH);
        let new_diff = diff * adjustment / distance;
        // let new_diff = diff;

        let angle = Vec2::new(0., 1.).angle_between(new_diff);
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

#[derive(Bundle)]
pub struct MovementInteruptBundle {
    effect_marker: EffectMarker,
    interupt: InteruptEffect,
    target: EffectTarget,
}

impl MovementInteruptBundle {
    pub fn new<T: Into<EffectTarget>>(target: T) -> Self {
        Self {
            effect_marker: EffectMarker,
            interupt: InteruptEffect::default(),
            target: target.into(),
        }
    }
}

/// Receives [`MotionKey`] input and accelerates character in said direction.
pub fn player_movement(
    motion_input: Res<Input<MotionKey>>,

    // CharacterIndex query
    mut character_query: Query<
        (
            Entity,
            &CharacterSpeedMultiplier,
            &mut Transform,
            &mut Velocity,
        ),
        With<PlayerMarker>,
    >,

    mut commands: Commands,
) {
    let (character_entity, speed_multiplier, transform, mut velocity) =
        character_query.single_mut().expect("player not found");

    const FORWARD_SPEED: f32 = 1.0;
    const STRAFE_SPEED: f32 = 0.8;
    const BACKPEDDLE_SPEED: f32 = 0.6;

    // Construct direction
    let mut direction = Vec2::ZERO;
    if motion_input.pressed(MotionKey::Left) {
        direction.x -= STRAFE_SPEED;
    }

    if motion_input.pressed(MotionKey::Forward) {
        direction.y += FORWARD_SPEED;
    }

    if motion_input.pressed(MotionKey::Right) {
        direction.x += STRAFE_SPEED;
    }

    if motion_input.pressed(MotionKey::Backward) {
        direction.y -= BACKPEDDLE_SPEED;
    }

    if direction != Vec2::ZERO {
        // Normalize
        let mag = direction.length().max(1.);
        direction /= mag;

        commands
            .spawn()
            .insert_bundle(MovementInteruptBundle::new(character_entity));
    }

    let direction = transform.rotation * (direction.extend(0.));

    // Assign velocity
    *velocity = Velocity::from(direction * speed_multiplier.speed());
}

/// Receives an [`ActionKey`] and performs the associated action.
pub fn player_action(
    time: Res<Time>,
    physics_world: PhysicsWorld,

    // ActionKey events
    mut events: EventReader<ActionKey>,

    mut character_query: Query<
        (
            Entity,
            &Transform,
            &CharacterClass,
            &LastCastInstant,
            &mut CharacterCastState,
            &CharacterTarget,
        ),
        With<PlayerMarker>,
    >,

    target_query: Query<&Transform, With<CharacterMarker>>,
) {
    let (
        character_entity,
        character_transform,
        character_class,
        last_cast_instant,
        mut character_cast_state,
        character_target,
    ) = character_query.single_mut().expect("player not found");

    for action_key in events.iter() {
        match character_class {
            CharacterClass::Mars => {}
            CharacterClass::Medea => match action_key {
                ActionKey::Action1 => {
                    let result = fireball_action(
                        &time,
                        &physics_world,
                        &last_cast_instant,
                        character_entity,
                        character_transform,
                        &character_target,
                        &mut character_cast_state,
                        &target_query,
                    );

                    if let Err(error) = result {
                        warn!(message = "failed to cast fireball", %error)
                    }
                }
                ActionKey::Action2 => todo!(),
                ActionKey::Action3 => todo!(),
                ActionKey::Action4 => todo!(),
                ActionKey::Action5 => todo!(),
                ActionKey::Action6 => todo!(),
                ActionKey::Action7 => todo!(),
                ActionKey::Action8 => todo!(),
            },
            CharacterClass::Heka => {}
            CharacterClass::Pluto => {}
            CharacterClass::Mammon => {}
        }
    }
}
