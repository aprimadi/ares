use std::time::Duration;

use mq::{input, window};

mod assets;
mod error;
mod screen;
mod utils;

type AResult<T = ()> = Result<T, error::AError>;

struct MainState {
    screen: screen::ScreenStack,
}

impl MainState {
    fn new() -> AResult<Self> {
        let start_screen = Box::new(screen::MainMenu::new()?);
        let screen = screen::ScreenStack::new(start_screen)?;
        Ok(Self { screen })
    }

    fn tick(&mut self) -> AResult {
        // Handle possible window resize and create a camera.
        let aspect_ratio = utils::aspect_ratio();
        let camera = utils::make_and_set_camera(aspect_ratio);
        self.screen.resize(aspect_ratio)?;
        // Handle user input events.
        let pos = utils::get_world_mouse_pos(&camera);
        self.screen.move_mouse(pos)?;
        if input::is_mouse_button_pressed(input::MouseButton::Left) {
            self.screen.click(pos)?;
        }
        // Update the game state.
        let dtime = Duration::from_secs_f32(mq::time::get_frame_time());
        self.screen.update(dtime)?;
        // Draw everything.
        mq::window::clear_background(screen::COLOR_SCREEN_BG);
        self.screen.draw()?;

        Ok(())
    }
}

fn window_conf() -> window::Conf {
    window::Conf {
        window_title: "Ares".to_owned(),
        high_dpi: true,
        ..Default::default()
    }
}

#[mq::main(window_conf)]
#[macroquad(crate_rename = "mq")]
async fn main() -> AResult {
    let mut state = MainState::new()?;
    loop {
        state.tick().expect("tick failed");
        window::next_frame().await;
    }
    // TODO
}
