mod damage;
mod interrupt;
mod power_burn;
mod trigger_global_cooldown;

use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use bevy::{
    ecs::{
        query::{Fetch, WorldQuery},
        system::{SystemParam, SystemParamFetch},
    },
    prelude::*,
};

use crate::{
    abilities::{Source, Target},
    character::CharacterMarker,
};

pub use damage::*;
pub use interrupt::*;
pub use power_burn::*;
pub use trigger_global_cooldown::*;

/// A marker component.
#[derive(Debug, Default, Clone, Component)]
pub struct EffectMarker;

type EffectParamItem<'world, 'state, E> =
    <<E as Effect>::Fetch as SystemParamFetch<'world, 'state>>::Item;

/// Represents an effect over the course of a single frame.
pub trait Effect
where
    for<'w, 's> EffectParamItem<'w, 's, Self>: SystemParam<Fetch = Self::Fetch>,
{
    /// The domain of the character query.
    type Domain<'a>: WorldQuery;

    /// Auxillary [`SystemParam`].
    type Param<'w, 's>: SystemParam<Fetch = Self::Fetch>;
    /// The [`Fetch`] of `Self::Param`.
    type Fetch: for<'w, 's> SystemParamFetch<'w, 's>;

    fn apply(
        &self,
        time: &Time,
        item: <<Self::Domain<'_> as WorldQuery>::Fetch as Fetch>::Item,
        param: &<Self::Fetch as SystemParamFetch>::Item,
        commands: &mut Commands,
    );
}

/// The [`Effect`] is aimed at current target.
#[derive(Debug, Clone, Component)]
pub struct AtTarget<T>(pub T);

/// Applies an [`Effect`] to some target.
pub fn apply_effect_target<E>(
    time: Res<Time>,
    ability_query: Query<(&AtTarget<E>, &Target), With<EffectMarker>>,
    mut character_query: Query<<E as Effect>::Domain<'_>, With<CharacterMarker>>,
    sys_param: <E::Fetch as SystemParamFetch>::Item,

    mut commands: Commands,
) where
    E: Send + Sync + 'static,
    E: Effect,
    for<'w, 's> <E::Fetch as SystemParamFetch<'w, 's>>::Item: SystemParam<Fetch = E::Fetch>,
{
    for (AtTarget(effect), target) in ability_query.iter() {
        let item = character_query
            .get_mut(target.0)
            .expect("failed to find target");
        effect.apply(&time, item, &sys_param, &mut commands);
    }
}

/// The [`Effect`] is aimed at self.
#[derive(Debug, Clone, Component)]
pub struct AtSelf<T>(pub T);

/// Applies an [`Effect`] to self.
pub fn apply_effect_self<E>(
    time: Res<Time>,
    ability_query: Query<(&AtSelf<E>, &Source), With<EffectMarker>>,
    mut character_query: Query<<E as Effect>::Domain<'_>, With<CharacterMarker>>,
    sys_param: EffectParamItem<'_, '_, E>,

    mut commands: Commands,
) where
    E: Send + Sync + 'static,
    E: Effect,
    for<'w, 's> EffectParamItem<'w, 's, E>: SystemParam<Fetch = E::Fetch>,
{
    for (AtSelf(effect), source) in ability_query.iter() {
        let item = character_query
            .get_mut(source.0)
            .expect("failed to find target");
        effect.apply(&time, item, &sys_param, &mut commands);
    }
}

/// The [`Effect`] will hit everyone in an AOE.
#[derive(Debug, Clone, Component)]
pub struct AtAoe<T> {
    effect: T,
    radius: f32,
}

/// Applies an [`Effect`] to everyone in a given radius.
pub fn apply_effect_radius<E>(// ability_query: Query<(&AtAoe<E>, &Transform), With<EffectMarker>>,
    // mut character_query: Query<<E as Effect>::Domain, With<CharacterMarker>>,
)
where
    E: Send + Sync + 'static,
    E: Effect,
    for<'w, 's> EffectParamItem<'w, 's, E>: SystemParam<Fetch = E::Fetch>,
{
    // TODO
}

/// Represents a single [`Effect`] subsystem.
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
    E: Effect,
    for<'w, 's> <E::Fetch as SystemParamFetch<'w, 's>>::Item: SystemParam<Fetch = E::Fetch>,
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

/// Remove all the [`Effect`]s after they are applied.
pub fn cleanup(query: Query<Entity, With<EffectMarker>>, mut commands: Commands) {
    for instance_id in query.iter() {
        info!("cleaning up instant effect");
        commands.entity(instance_id).despawn();
    }
}

/// Aggregate all the [`Effect`] subsystems.
pub struct EffectPlugin<T, L> {
    pub state: T,
    pub label: L,
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
        let trigger_global_cooldown_effects =
            SingleEffectPlugin::<_, _, TriggerGlobalCooldown>::new(self.state, self.label.clone());
        let interupt_effects =
            SingleEffectPlugin::<_, _, Interrupt>::new(self.state, self.label.clone());

        let cleanup_set = SystemSet::on_update(self.state)
            .after(self.label.clone())
            .with_system(cleanup);

        app.add_plugin(damage_effects)
            .add_plugin(power_burn_effects)
            .add_plugin(trigger_global_cooldown_effects)
            .add_plugin(interupt_effects)
            .add_system_set(cleanup_set);
    }
}
