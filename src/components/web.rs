use bevy::prelude::*;

#[derive(Component, Default, Reflect)]
pub struct WebString;

#[derive(Component, Default, Reflect)]
pub struct WebHead {
    pub stuck_on_wall: bool,
}

#[derive(Component, Default, Reflect)]
pub struct Web;
