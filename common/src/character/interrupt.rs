use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use bevy::{prelude::*, utils::Instant};

use crate::abilities::magic_school::{Fire, Frost, Nature};

#[derive(Debug, Component)]
pub struct Interrupted<School> {
    _school: PhantomData<School>,
    pub start: Instant,
    pub until: Instant,
}

impl<School> Interrupted<School> {
    pub fn new(start: Instant, until: Instant) -> Self {
        Self {
            _school: PhantomData,
            start,
            until,
        }
    }
}

pub fn interrupted_cleanup<School: Component>(
    time: Res<Time>,
    query: Query<(Entity, &Interrupted<School>)>,
    mut commands: Commands,
) {
    let now = time.last_update().expect("cant find last update");
    for (id, interrupted) in query.iter() {
        if interrupted.until < now {
            commands.entity(id).remove::<Interrupted<School>>();
        }
    }
}

pub struct InterruptedPlugin<T> {
    pub state: T,
}

impl<T> Plugin for InterruptedPlugin<T>
where
    T: Send + Sync + 'static,
    T: Eq + Debug + Hash + Clone,
{
    fn build(&self, app: &mut App) {
        let set = SystemSet::on_update(self.state.clone())
            .with_system(interrupted_cleanup::<Fire>)
            .with_system(interrupted_cleanup::<Frost>)
            .with_system(interrupted_cleanup::<Nature>);
        app.add_system_set(set);
    }
}
