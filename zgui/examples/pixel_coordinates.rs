use mq::{
    camera::{set_camera, Camera2D},
    color::{RED, WHITE},
    math::Rect,
};
use zgui as ui;

mod common;

#[derive(Clone, Copy, Debug)]
enum Message {
    Command,
}

pub fn make_and_set_camera(_aspect_ratio: f32) -> Camera2D {
    let display_rect = Rect {
        x: 0.0,
        y: 0.0,
        w: mq::window::screen_width(),
        h: mq::window::screen_height(),
    };
    let camera = Camera2D::from_display_rect(display_rect);
    set_camera(&camera);
    camera
}

fn make_gui(font: mq::text::Font) -> ui::Result<ui::Gui<Message>> {
    let mut gui = ui::Gui::new();
    let anchor = ui::Anchor(ui::HAnchor::Right, ui::VAnchor::Bottom);
    let text = ui::Drawable::text("Button", font);
    let button = ui::Button::new(text, 0.2, gui.sender(), Message::Command)?;
    gui.add(&ui::pack(button), anchor);
    Ok(gui)
}

fn draw_scene() {
    let x = 150.0;
    let y = 150.0;
    let r = 100.0;
    let color = RED;
    mq::shapes::draw_circle(x, y, r, color);
}

#[mq::main("ZGui: Pixel Coordinates Demo")]
#[macroquad(crate_rename = "mq")]
async fn main() {
    let assets = common::Assets::load().await.expect("Can't load assets");
    let mut gui = make_gui(assets.font).expect("Can't create the gui");
    loop {
        // Update the camera and the GUI.
        let aspect_ratio = common::aspect_ratio();
        let camera = make_and_set_camera(aspect_ratio);
        gui.resize_if_needed(aspect_ratio);
        // Handle cursor updates.
        let pos = common::get_world_mouse_pos(&camera);
        gui.move_mouse(pos);
        if mq::input::is_mouse_button_pressed(mq::input::MouseButton::Left) {
            let message = gui.click(pos);
            println!("{:?}", message);
        }
        // Draw the GUI.
        mq::window::clear_background(WHITE);
        draw_scene();
        gui.draw();
        mq::window::next_frame().await;
    }
}
