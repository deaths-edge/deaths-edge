use std::{fmt, sync::Arc};

use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::abilities::Source;

#[derive(Component, Clone)]
pub struct DynEntityMutate(Arc<dyn Fn(Entity, &mut EntityCommands) + Send + Sync>);

impl fmt::Debug for DynEntityMutate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DynEntityMutate").finish()
    }
}

impl DynEntityMutate {
    pub fn apply(&self, parent: Entity, entity_commands: &mut EntityCommands) {
        self.0(parent, entity_commands);
    }
}

#[derive(Component, Clone)]
pub struct EntityMutate<F>(F);

impl EntityMutate<fn(Entity, &mut EntityCommands)> {
    pub fn new() -> Self {
        Self(|_, _| ())
    }
}

impl<F> EntityMutate<F> {
    /// Insert a [`Bundle`] into new entity.
    pub fn insert_bundle<T>(self, bundle: T) -> EntityMutate<impl Fn(Entity, &mut EntityCommands)>
    where
        F: Fn(Entity, &mut EntityCommands),
        T: Bundle + Clone,
    {
        let new_f = move |entity: Entity, entity_commands: &mut EntityCommands| {
            (self.0)(entity, entity_commands);
            entity_commands.insert_bundle(bundle.clone());
        };
        EntityMutate(new_f)
    }

    /// Inserts [`Source`] as the parent entity.
    pub fn parent_source(self) -> EntityMutate<impl Fn(Entity, &mut EntityCommands)>
    where
        F: Fn(Entity, &mut EntityCommands),
    {
        let new_f = move |entity: Entity, entity_commands: &mut EntityCommands| {
            (self.0)(entity, entity_commands);
            entity_commands.insert(Source(entity));
        };
        EntityMutate(new_f)
    }

    /// Move a [`Component`] from parent to this entity.
    pub fn snapshot_move<T>(
        self,
    ) -> EntityMutate<impl Fn(Entity, &mut EntityCommands) + Send + Sync + 'static>
    where
        F: Fn(Entity, &mut EntityCommands) + Send + Sync + 'static,
        T: Component + Clone,
    {
        let new_f = move |parent: Entity, entity_commands: &mut EntityCommands| {
            (self.0)(parent, entity_commands);

            let target = entity_commands.id();
            entity_commands
                .commands()
                .entity(parent)
                .r#move::<T>(target);
        };
        EntityMutate(new_f)
    }

    /// Clone a [`Component`] from parent to this entity.
    pub fn snapshot_clone<T>(
        self,
    ) -> EntityMutate<impl Fn(Entity, &mut EntityCommands) + Send + Sync + 'static>
    where
        F: Fn(Entity, &mut EntityCommands) + Send + Sync + 'static,
        T: Component + Clone,
    {
        let new_f = move |parent: Entity, entity_commands: &mut EntityCommands| {
            (self.0)(parent, entity_commands);

            let target = entity_commands.id();
            entity_commands
                .commands()
                .entity(parent)
                .r#clone::<T>(target);
        };
        EntityMutate(new_f)
    }
}

impl<F> EntityMutate<F>
where
    F: Fn(Entity, &mut EntityCommands) + Send + Sync + 'static,
{
    pub fn arc(self) -> DynEntityMutate {
        DynEntityMutate(Arc::new(self.0))
    }

    pub fn apply(&self, parent: Entity, entity_commands: &mut EntityCommands) {
        self.0(parent, entity_commands);
    }
}
