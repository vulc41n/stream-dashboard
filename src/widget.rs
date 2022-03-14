use tui::backend::Backend;
use tui::layout::Rect;
use tui::terminal::Frame;

pub trait AppWidget {
  fn draw<T: Backend>(&mut self, f: &mut Frame<T>, area: Rect);
}

