use bevy::{ecs::system::EntityCommands, prelude::*};

#[derive(Component)]
pub struct DynCommand(Box<dyn Fn(&mut EntityCommands) + Send + Sync>);

impl DynCommand {
    pub fn insert_bundle<T: Bundle + Clone>(bundle: T) -> Self {
        Self(Box::new(move |commands| {
            commands.insert_bundle(bundle.clone());
        }))
    }

    pub fn apply(&self, entity_commands: &mut EntityCommands) {
        self.0(entity_commands);
    }
}
