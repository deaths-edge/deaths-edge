pub mod damage;
pub mod power_burn;

use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use bevy::{
    ecs::query::{Fetch, WorldQuery},
    prelude::*,
};

use crate::{
    abilities::{Source, Target},
    character::CharacterMarker,
};

use damage::Damage;
use power_burn::PowerBurn;

/// Components accompanied by this will take effect immediately.
#[derive(Component)]
pub struct EffectMarker;

pub trait Effect<'a> {
    type Domain: WorldQuery + 'a;

    fn apply(&self, item: <<Self::Domain as WorldQuery>::Fetch as Fetch>::Item);
}

#[derive(Debug, Clone, Component)]
pub struct AtTarget<T>(pub T);

/// Applies an [`Effect`] to some target.
pub fn apply_effect_target<E>(
    ability_query: Query<(&AtTarget<E>, &Target), With<EffectMarker>>,
    mut character_query: Query<<E as Effect>::Domain, With<CharacterMarker>>,
) where
    E: Send + Sync + 'static,
    for<'a> E: Effect<'a>,
{
    for (AtTarget(effect), target) in ability_query.iter() {
        let item = character_query
            .get_mut(target.0)
            .expect("failed to find target");
        effect.apply(item);
    }
}

#[derive(Debug, Clone, Component)]
pub struct AtSelf<T>(pub T);

/// Applies an [`Effect`] to self.
pub fn apply_effect_self<E>(
    ability_query: Query<(&AtSelf<E>, &Source), With<EffectMarker>>,
    mut character_query: Query<<E as Effect>::Domain, With<CharacterMarker>>,
) where
    E: Send + Sync + 'static,
    for<'a> E: Effect<'a>,
{
    for (AtSelf(effect), source) in ability_query.iter() {
        let item = character_query
            .get_mut(source.0)
            .expect("failed to find target");
        effect.apply(item);
    }
}

#[derive(Debug, Clone, Component)]
pub struct AtAoe<T>(pub T);

/// Applies an [`Effect`] to everyone in a given radius.
pub fn apply_effect_radius<E>(
    ability_query: Query<(&AtAoe<E>, &Transform), With<EffectMarker>>,
    mut character_query: Query<<E as Effect>::Domain, With<CharacterMarker>>,
) where
    E: Send + Sync + 'static,
    for<'a> E: Effect<'a>,
{
    // TODO
}

struct SingleEffectPlugin<T, L, E> {
    state: T,
    label: L,
    _effect: PhantomData<E>,
}

impl<T, L, E> SingleEffectPlugin<T, L, E> {
    fn new(state: T, label: L) -> Self {
        Self {
            state,
            label,
            _effect: PhantomData,
        }
    }
}

impl<T, L, E> Plugin for SingleEffectPlugin<T, L, E>
where
    T: Send + Sync + 'static,
    T: Debug + Clone + Copy + Eq + Hash,

    L: Send + Sync + 'static,
    L: SystemLabel + Clone,

    E: Send + Sync + 'static,
    for<'a> E: Effect<'a>,
{
    fn build(&self, app: &mut App) {
        let set = SystemSet::on_update(self.state)
            .label(self.label.clone())
            .with_system(apply_effect_target::<E>)
            .with_system(apply_effect_self::<E>)
            .with_system(apply_effect_radius::<E>);

        app.add_system_set(set);
    }
}

pub struct EffectPlugin<T, L> {
    state: T,
    label: L,
}

impl<T, L> EffectPlugin<T, L> {
    pub fn new(state: T, label: L) -> Self {
        Self { state, label }
    }
}

impl<T, L> Plugin for EffectPlugin<T, L>
where
    T: Send + Sync + 'static,
    T: Debug + Clone + Copy + Eq + Hash,

    L: Send + Sync + 'static,
    L: SystemLabel + Clone,
{
    fn build(&self, app: &mut App) {
        let damage_effects =
            SingleEffectPlugin::<_, _, Damage>::new(self.state, self.label.clone());
        let power_burn_effects =
            SingleEffectPlugin::<_, _, PowerBurn>::new(self.state, self.label.clone());

        let cleanup_set = SystemSet::on_update(self.state)
            .after(self.label.clone())
            .with_system(cleanup);

        app.add_plugin(damage_effects)
            .add_plugin(power_burn_effects)
            .add_system_set(cleanup_set);
    }
}

pub fn cleanup(query: Query<Entity, With<EffectMarker>>, mut commands: Commands) {
    for instance_id in query.iter() {
        info!("cleaning up instant effect");
        commands.entity(instance_id).despawn();
    }
}
