use std::marker::PhantomData;

use bevy::{ecs::schedule::StateData, prelude::*};
use iyes_loopless::prelude::AppLooplessStateExt;
use strum::IntoEnumIterator;

use crate::data::clean_up::Persist;

pub struct CleanUpPlugin<State>
where
    State: StateData + IntoEnumIterator,
{
    s: PhantomData<State>,
}
impl<State> Default for CleanUpPlugin<State>
where
    State: StateData + IntoEnumIterator,
{
    fn default() -> Self {
        Self {
            s: Default::default(),
        }
    }
}
impl<State> Plugin for CleanUpPlugin<State>
where
    State: StateData + IntoEnumIterator,
{
    fn build(&self, app: &mut App) {
        for state in State::iter() {
            app
            .add_startup_system_to_stage(StartupStage::PostStartup, mark_persist_on_startup)
            .add_exit_system(state, clean_up_system);
        }
    }
}

fn clean_up_system(mut commands: Commands, q: Query<Entity, (Without<Persist>, Without<Parent>)>) {
    q.for_each(|e| {
        commands.entity(e).despawn_recursive();
    })
}

fn mark_persist_on_startup(mut commands: Commands, q: Query<Entity>) {
    q.for_each(|e| {
        commands.entity(e).insert(Persist);
    })
}
