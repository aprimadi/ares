use std::{fmt::Debug, time::Duration};

pub use crate::action::{
    change_color_to::ChangeColorTo, custom::Custom, empty::Empty, fork::Fork, hide::Hide,
    move_by::MoveBy, sequence::Sequence, set_color::SetColor, set_facing::SetFacing,
    set_frame::SetFrame, show::Show, sleep::Sleep,
};

mod change_color_to;
mod custom;
mod empty;
mod fork;
mod hide;
mod move_by;
mod sequence;
mod set_color;
mod set_facing;
mod set_frame;
mod show;
mod sleep;

pub trait Action: Debug {
    fn begin(&mut self) {}
    fn update(&mut self, _dtime: Duration) {}
    fn end(&mut self) {}

    /// Note that it return only the main actions' duration and ignores all forks.
    /// Also see [Scene::any_unfinished_actions] if you need to check for alive forks.
    fn duration(&self) -> Duration {
        Duration::new(0, 0)
    }

    fn try_fork(&mut self) -> Option<Box<dyn Action>> {
        None
    }

    fn is_finished(&self) -> bool {
        true
    }
}

/// Just a helper trait to replace
/// `Box::new(action::Empty::new())`
/// with
/// `action::Empty::new().boxed()`.
pub trait Boxed {
    type Out;

    fn boxed(self) -> Self::Out;
}

impl<T: 'static + Action> Boxed for T {
    type Out = Box<dyn Action>;

    fn boxed(self) -> Self::Out {
        Box::new(self)
    }
}
