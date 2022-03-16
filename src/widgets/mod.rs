use tui::backend::Backend;
use tui::layout::Rect;
use tui::terminal::Frame;

mod jukebox;
pub use jukebox::Jukebox;
mod status;
pub use status::StatusBar;

pub trait AppWidget {
  fn draw<T: Backend>(&mut self, f: &mut Frame<T>, area: Rect);
}

