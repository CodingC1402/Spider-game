use std::marker::PhantomData;

use bevy::{prelude::*, utils::Uuid};

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

#[derive(Default)]
pub struct AnimPlugin<R, T>
where
    R: AnimTreeWrap<T>,
    T: AnimState,
{
    phantom_t: PhantomData<T>,
    phantom_r: PhantomData<R>,
}
impl<R, T> Plugin for AnimPlugin<R, T>
where
    R: AnimTreeWrap<T>,
    T: AnimState,
{
    fn build(&self, app: &mut App) {
        app.add_system(anim_tree_update::<R, T>);
    }
}
