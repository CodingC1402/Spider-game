use std::marker::PhantomData;

use bevy::{ecs::schedule::StateData, prelude::*};
use iyes_loopless::prelude::{AppLooplessStateExt, ConditionHelpers, IntoConditionalSystem};
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
            app.add_exit_system(state, clean_up_system);
        }
    }
}

fn clean_up_system(mut commands: Commands, q: Query<Entity, Without<Persist>>) {
    q.for_each(|e| {
        commands.entity(e).despawn_recursive();
    })
}
