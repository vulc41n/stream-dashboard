use tui::widgets::Block;

pub trait AppWidget {
  fn draw(&self) -> Block;
}
