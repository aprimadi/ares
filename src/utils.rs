use mq::window;
use mq::camera::{set_camera, Camera2D};
use mq::math::{Rect, Vec2};

pub fn aspect_ratio() -> f32 {
    window::screen_width() / window::screen_height()
}

pub fn make_and_set_camera(aspect_ratio: f32) -> Camera2D {
    let camera = Camera2D::from_display_rect(Rect {
        x: -aspect_ratio,
        y: -1.0,
        w: aspect_ratio * 2.0,
        h: 2.0,
    });
    set_camera(&camera);
    camera
}

pub fn get_world_mouse_pos(camera: &Camera2D) -> Vec2 {
    camera.screen_to_world(mq::input::mouse_position().into())
}

