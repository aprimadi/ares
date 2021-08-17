use std::time::Duration;

use mq::math::Vec2;
use ui::{Gui, Widget};

use crate::AResult;
use crate::assets;
use crate::screen::{Screen, StackCommand};
use crate::utils;

#[derive(Copy, Clone, Debug)]
enum Action {
    Exit,
    StartCampaign,
}

fn make_gui() -> AResult<ui::Gui<Action>> {
    let font = assets::get().font;
    let mut gui = ui::Gui::new();
    let h = utils::line_heights().large;
    let space = || Box::new(ui::Spacer::new_vertical(h / 8.0));
    let button = &mut |text, message| -> AResult<_> {
        let text = ui::Drawable::text(text, font);
        let b = ui::Button::new(text, h, gui.sender(), message)?.stretchable(true);
        Ok(Box::new(b))
    };
    let mut layout = Box::new(ui::VLayout::new().stretchable(true));
    layout.add(button("campaign", Action::StartCampaign)?);
    #[cfg(not(target_arch = "wasm32"))] // can't quit WASM
    {
        layout.add(space());
        layout.add(button("exit", Action::Exit)?);
    }
    layout.stretch_to_self();
    let layout = utils::add_offsets_and_bg_big(layout)?;
    let anchor = ui::Anchor(ui::HAnchor::Middle, ui::VAnchor::Middle);
    gui.add(&ui::pack(layout), anchor);
    Ok(gui)
}

#[derive(Debug)]
pub struct MainMenu {
    gui: Gui<Action>,
}

// TODO: add the game's version to one of the corners
impl MainMenu {
    pub fn new() -> AResult<Self> {
        let gui = make_gui()?;
        Ok(Self { gui })
    }
}

impl Screen for MainMenu {
    fn update(&mut self, _: Duration) -> AResult<StackCommand> {
        Ok(StackCommand::None)
    }

    fn draw(&self) -> AResult {
        self.gui.draw();
        Ok(())
    }

    fn click(&mut self, pos: Vec2) -> AResult<StackCommand> {
        let action = self.gui.click(pos);
        log::trace!("MainMenu: click: pos={:?}, message={:?}", pos, action);
        match action {
            Some(Action::StartCampaign) => {
                /*
                let screen = screen::Campaign::new()?;
                Ok(StackCommand::PushScreen(Box::new(screen)))
                */
                Ok(StackCommand::None)
            }
            Some(Action::Exit) => Ok(StackCommand::Pop),
            None => Ok(StackCommand::None),
        }
    }

    fn resize(&mut self, aspect_ratio: f32) {
        self.gui.resize_if_needed(aspect_ratio);
    }

    fn move_mouse(&mut self, pos: Vec2) -> AResult {
        self.gui.move_mouse(pos);
        Ok(())
    }
}

