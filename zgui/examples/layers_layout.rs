use mq::color::WHITE;
use zgui as ui;

mod common;

#[derive(Clone, Copy, Debug)]
enum Message {
    Command,
}

fn make_gui(assets: common::Assets) -> ui::Result<ui::Gui<Message>> {
    let mut gui = ui::Gui::new();
    let text = ui::Drawable::text(" text", assets.font);
    let texture = ui::Drawable::Texture(assets.texture);
    let button = ui::Button::new(texture, 0.2, gui.sender(), Message::Command)?;
    let label = ui::Label::new(text, 0.1)?;
    let mut layout = ui::LayersLayout::new();
    layout.add(Box::new(button));
    layout.add(Box::new(label));
    let anchor = ui::Anchor(ui::HAnchor::Right, ui::VAnchor::Bottom);
    gui.add(&ui::pack(layout), anchor);
    Ok(gui)
}

#[mq::main("ZGui: Layers Layout Demo")]
#[macroquad(crate_rename = "mq")]
async fn main() {
    let assets = common::Assets::load().await.expect("Can't load assets");
    let mut gui = make_gui(assets).expect("Can't create the gui");
    loop {
        // Update the camera and the GUI.
        let aspect_ratio = common::aspect_ratio();
        let camera = common::make_and_set_camera(aspect_ratio);
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
        gui.draw();
        mq::window::next_frame().await;
    }
}
