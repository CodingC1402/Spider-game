use bevy::prelude::Component;

/// Mark entity with this for it to persist between game state
#[derive(Component)]
pub struct Persist;
