use bevy::prelude::*; 

pub struct Cursor;
pub struct CursorBundle {
    sprite: SpriteSheetBundle,
    cursor: Cursor
}