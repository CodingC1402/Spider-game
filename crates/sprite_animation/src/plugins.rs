use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{prelude::{AnimState, AnimTreeWrap}, systems::update::anim_tree_update};

#[derive(Component)]
pub struct AnimData<T> where T : AnimState {
    pub time: f32,
    pub index: usize,
    pub state: T
}

pub struct AnimPlugin<R, T> where R: AnimTreeWrap<T>, T: AnimState {
    phantom_t: PhantomData<T>,
    phantom_r: PhantomData<R>
}
impl<R, T> Plugin for AnimPlugin<R, T> where R: AnimTreeWrap<T>, T: AnimState {
    fn build(&self, app: &mut App) {
        app.add_system(anim_tree_update::<R, T>);
    }
}