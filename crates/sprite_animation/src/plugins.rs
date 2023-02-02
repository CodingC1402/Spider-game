use std::marker::PhantomData;

use bevy::{prelude::*, utils::Uuid, ecs::schedule::StateData};
use iyes_loopless::prelude::IntoConditionalSystem;

use crate::{
    prelude::{AnimState, AnimTreeWrap},
    systems::update::anim_tree_update,
};

#[derive(Component, Default)]
pub struct AnimData<T>
where
    T: AnimState,
{
    /// The mount of time since last frame
    pub time: f32,
    pub index: usize,
    pub current_node: Uuid,
    /// Used to update logic nodes, used like a stack
    pub logic_stack: Vec<(Uuid, usize)>,
    state: T,
    /// The amount of time passed since that state is set.
    state_time: f32,
}
impl<T> AnimData<T>
where
    T: AnimState,
{
    pub fn get_state(&self) -> &T {
        &self.state
    }

    pub fn get_state_time(&self) -> f32 {
        self.state_time
    }

    pub fn set_state(&mut self, state: T) {
        self.state_time = self.state.eq(&state).then_some(self.state_time).unwrap_or(0.0);
        self.state = state;
    }

    pub fn add_state_time(&mut self, delta_time: f32) {
        self.state_time += delta_time;
    }
}

pub struct AnimPlugin<R, T, S>
where
    R: AnimTreeWrap<T>,
    T: AnimState,
    S: StateData
{
    phantom_t: PhantomData<T>,
    phantom_r: PhantomData<R>,
    run_in: Option<S>
}
impl<R, T, S> AnimPlugin<R, T, S>
where
    R: AnimTreeWrap<T>,
    T: AnimState,
    S: StateData
{
    pub fn new(run_in: Option<S>) -> Self {
        Self {
            run_in,
            phantom_r: PhantomData,
            phantom_t: PhantomData
        }
    }
}
impl<R, T, S> Plugin for AnimPlugin<R, T, S>
where
    R: AnimTreeWrap<T>,
    T: AnimState,
    S: StateData
{
    fn build(&self, app: &mut App) {
        app
        .add_system(match self.run_in.clone() {
            Some(x) => anim_tree_update::<R, T>.run_in_state(x),
            None => anim_tree_update::<R, T>.into_conditional(),
        });
    }
}