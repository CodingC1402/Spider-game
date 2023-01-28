use bevy::prelude::*;

pub fn cursor_screen_to_world(
    // need to get window dimensions
    wnds: &Windows,
    // query to get camera transform
    camera: &Camera,
    camera_transform: &GlobalTransform,
) -> Option<Vec2> {
    // get the window that the camera is displaying to (or the primary window)
    let wnd = wnds.get_primary().unwrap();

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
        let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;

        // matrix for undoing the projection and camera transform
        let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

        // use it to convert ndc to world-space coordinates
        let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

        // reduce it to a 2D value
        let world_pos: Vec2 = world_pos.truncate();

        return Some(Vec2::new(world_pos.x, world_pos.y));
    }
    None
}
