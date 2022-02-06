mod apply_status;
mod damage;
mod dot;
mod interrupt;
mod power_burn;
mod trigger_cooldown;
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

pub use apply_status::*;
pub use damage::*;
pub use dot::*;
pub use interrupt::*;
pub use power_burn::*;
pub use trigger_cooldown::*;
pub use trigger_global_cooldown::*;

use super::{AbilityId, AbilityMarker};

/// Marks an [`Entity`] as a collection of "effects". These will be enacted every frame.
#[derive(Debug, Default, Clone, Component)]
pub struct EffectMarker;

type EffectParamItem<'world, 'state, E> =
    <<E as CharacterEffect>::Fetch as SystemParamFetch<'world, 'state>>::Item;

/// Represents an effect over the course of a single frame.
pub trait CharacterEffect
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

        parent_id: Entity,
        item: <<Self::Domain<'_> as WorldQuery>::Fetch as Fetch>::Item,
        param: &<Self::Fetch as SystemParamFetch>::Item,

        time: &Time,

        commands: &mut Commands,
    );
}

pub trait AbilityEffect {
    /// The domain of the ability query.
    type Domain<'a>: WorldQuery;

    fn apply(
        &self,
        time: &Time,
        item: <<Self::Domain<'_> as WorldQuery>::Fetch as Fetch>::Item,
        commands: &mut Commands,
    );
}

/// Apply a [`AbilityEffect`].
#[derive(Debug, Clone, Component)]
pub struct AtAbility<T>(pub T);

pub fn apply_ability<E>(
    time: Res<Time>,
    effect_query: Query<(&AtAbility<E>, &AbilityId), With<EffectMarker>>,
    mut ability_query: Query<<E as AbilityEffect>::Domain<'_>, With<AbilityMarker>>,
    mut commands: Commands,
) where
    E: Send + Sync + 'static,
    E: AbilityEffect,
{
    for (AtAbility(effect), AbilityId(ability_id)) in effect_query.iter() {
        let item = ability_query
            .get_mut(*ability_id)
            .expect("failed to find target");
        effect.apply(&time, item, &mut commands);
    }
}

/// The [`CharacterEffect`] is aimed at current target.
#[derive(Debug, Clone, Component)]
pub struct AtTarget<T>(pub T);

/// Applies an [`CharacterEffect`] to some target.
pub fn apply_effect_target<E>(
    time: Res<Time>,
    effect_query: Query<(Entity, &AtTarget<E>, &Target), With<EffectMarker>>,
    mut character_query: Query<<E as CharacterEffect>::Domain<'_>, With<CharacterMarker>>,
    sys_param: <E::Fetch as SystemParamFetch>::Item,

    mut commands: Commands,
) where
    E: Send + Sync + 'static,
    E: CharacterEffect,
    for<'w, 's> <E::Fetch as SystemParamFetch<'w, 's>>::Item: SystemParam<Fetch = E::Fetch>,
{
    for (parent_id, AtTarget(effect), Target(target)) in effect_query.iter() {
        let item = character_query
            .get_mut(*target)
            .expect("failed to find target");
        effect.apply(parent_id, item, &sys_param, &time, &mut commands);
    }
}

/// The [`CharacterEffect`] is aimed at self.
#[derive(Debug, Clone, Component)]
pub struct AtSelf<T>(pub T);

/// Applies an [`CharacterEffect`] to self.
pub fn apply_effect_self<E>(
    time: Res<Time>,
    effect_query: Query<(Entity, &AtSelf<E>, &Source), With<EffectMarker>>,
    mut character_query: Query<<E as CharacterEffect>::Domain<'_>, With<CharacterMarker>>,
    sys_param: EffectParamItem<'_, '_, E>,

    mut commands: Commands,
) where
    E: Send + Sync + 'static,
    E: CharacterEffect,
    for<'w, 's> EffectParamItem<'w, 's, E>: SystemParam<Fetch = E::Fetch>,
{
    for (parent_id, AtSelf(effect), Source(source)) in effect_query.iter() {
        let item = character_query
            .get_mut(*source)
            .expect("failed to find target");
        effect.apply(parent_id, item, &sys_param, &time, &mut commands);
    }
}

// /// The [`CharacterEffect`] will hit everyone in an AOE.
// #[derive(Debug, Clone, Component)]
// pub struct AtAoe<T> {
//     // effect: T,
//     // radius: f32,
// }

// /// Applies an [`CharacterEffect`] to everyone in a given radius.
// pub fn apply_effect_radius<E>(// ability_query: Query<(&AtAoe<E>, &Transform), With<EffectMarker>>,
//     // mut character_query: Query<<E as CharacterEffect>::Domain, With<CharacterMarker>>,
// )
// where
//     E: Send + Sync + 'static,
//     E: CharacterEffect,
//     for<'w, 's> EffectParamItem<'w, 's, E>: SystemParam<Fetch = E::Fetch>,
// {
//     // TODO
// }

/// Represents a single [`CharacterEffect`] subsystem.
struct CharacterEffectPlugin<T, L, E> {
    state: T,
    label: L,
    _effect: PhantomData<E>,
}

impl<T, L, E> CharacterEffectPlugin<T, L, E> {
    fn new(state: T, label: L) -> Self {
        Self {
            state,
            label,
            _effect: PhantomData,
        }
    }
}

impl<T, L, E> Plugin for CharacterEffectPlugin<T, L, E>
where
    T: Send + Sync + 'static,
    T: Debug + Clone + Copy + Eq + Hash,

    L: Send + Sync + 'static,
    L: SystemLabel + Clone,

    E: Send + Sync + 'static,
    E: CharacterEffect,
    for<'w, 's> <E::Fetch as SystemParamFetch<'w, 's>>::Item: SystemParam<Fetch = E::Fetch>,
{
    fn build(&self, app: &mut App) {
        let set = SystemSet::on_update(self.state)
            .label(self.label.clone())
            .with_system(apply_effect_target::<E>)
            .with_system(apply_effect_self::<E>)
            // .with_system(apply_effect_radius::<E>)
            ;

        app.add_system_set(set);
    }
}

pub struct AbilityEffectPlugin<T, L, E> {
    state: T,
    label: L,
    _effect: PhantomData<E>,
}

impl<T, L, E> AbilityEffectPlugin<T, L, E> {
    fn new(state: T, label: L) -> Self {
        Self {
            state,
            label,
            _effect: PhantomData,
        }
    }
}

impl<T, L, E> Plugin for AbilityEffectPlugin<T, L, E>
where
    T: Send + Sync + 'static,
    T: Debug + Clone + Copy + Eq + Hash,

    L: Send + Sync + 'static,
    L: SystemLabel + Clone,

    E: Send + Sync + 'static,
    E: AbilityEffect,
{
    fn build(&self, app: &mut App) {
        let set = SystemSet::on_update(self.state)
            .label(self.label.clone())
            .with_system(apply_ability::<E>);

        app.add_system_set(set);
    }
}

/// Aggregate all the [`CharacterEffect`] subsystems.
pub struct EffectPlugin<T, L> {
    pub state: T,
    pub label: L,
}

impl<T, L> EffectPlugin<T, L>
where
    T: Send + Sync + 'static,
    T: Debug + Clone + Copy + Eq + Hash,

    L: SystemLabel + Clone,
{
    pub fn add_character_effect<E>(&self, app: &mut App) -> &Self
    where
        E: Sync + Send + 'static,
        E: CharacterEffect,
        for<'w, 's> <E::Fetch as SystemParamFetch<'w, 's>>::Item: SystemParam<Fetch = E::Fetch>,
    {
        let plugin = CharacterEffectPlugin::<_, _, E>::new(self.state, self.label.clone());
        app.add_plugin(plugin);
        self
    }

    pub fn add_ability_effect<E>(&self, app: &mut App) -> &Self
    where
        E: Sync + Send + 'static,
        E: AbilityEffect,
    {
        let plugin = AbilityEffectPlugin::<_, _, E>::new(self.state, self.label.clone());
        app.add_plugin(plugin);
        self
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
        self.add_character_effect::<Damage>(app)
            .add_character_effect::<PowerBurn>(app)
            .add_character_effect::<TriggerGlobalCooldown>(app)
            .add_character_effect::<Interrupt>(app)
            .add_character_effect::<Dot>(app)
            .add_character_effect::<ApplyStatus>(app)
            .add_ability_effect::<TriggerCooldown>(app);
    }
}
