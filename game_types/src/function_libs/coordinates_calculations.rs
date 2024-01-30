use bevy::math::Vec2;
use bevy::prelude::{Transform, Window};

// Convert function from screen space to world space
pub fn screen_to_world(pos: Vec2, window: &Window, camera: &Transform) -> Vec2 {
    // Get window size
    let window_size = Vec2::new(window.width() / 2.0, window.height() / 2.0);

    // Flip Y
    let flipped_pos = Vec2::new(pos.x, window.height() - pos.y);

    // Translate the coordinate system such that the origin is at the center of the screen
    let translated_pos = flipped_pos - window_size;

    // Scale and translate the point from screen space to world space
    let world_pos = translated_pos * camera.scale.truncate() + camera.translation.truncate();

    world_pos
}