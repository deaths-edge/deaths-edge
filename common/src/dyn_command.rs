use std::{fmt, sync::Arc};

use bevy::{ecs::system::EntityCommands, prelude::*};

#[derive(Component, Clone)]
pub struct DynCommand(Arc<dyn Fn(&mut EntityCommands) + Send + Sync>);

impl fmt::Debug for DynCommand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("DynCommand").finish()
    }
}

impl DynCommand {
    pub fn insert_bundle<T: Bundle + Clone>(bundle: T) -> Self {
        Self(Arc::new(move |commands| {
            commands.insert_bundle(bundle.clone());
        }))
    }

    pub fn apply(&self, entity_commands: &mut EntityCommands) {
        self.0(entity_commands);
    }
}
