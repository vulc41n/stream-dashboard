use tui::widgets::Widget;

pub trait AppWidget {
  fn draw() -> Box<dyn Widget>;
}
