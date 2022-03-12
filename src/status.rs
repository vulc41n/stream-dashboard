use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::{Block, Borders, Widget};

pub struct StatusBar {
  status: Option<Status>, 
}

impl StatusBar {
  pub fn new() -> Self {
    Self{ status: None }
  }
}

impl Widget for StatusBar {
  fn render(self, area: Rect, buf: &mut Buffer) {
    Block::default().borders(Borders::ALL).render(area, buf);
  }
}

pub struct Status {
}
