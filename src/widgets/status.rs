use tui::layout::Rect;
use tui::widgets::{Block, Borders, Paragraph};
use tui::text::{Spans, Span};
use tui::backend::Backend;
use tui::Frame;

use std::sync::mpsc::Receiver;

use crate::obs::Status;

use super::AppWidget;

pub struct StatusBar {
  status: Status,
  rx:     Receiver<Status>,
}

impl StatusBar {
  pub fn new(rx: Receiver<Status>) -> Self {
    Self{ status: Status::Offline, rx }
  }
}

impl AppWidget for StatusBar {
  fn draw<T: Backend>(&mut self, f: &mut Frame<T>, area: Rect) {
    if let Ok(status) = self.rx.try_recv() {
      self.status = status;
    }
    let block = Block::default()
      .borders(Borders::ALL)
      .title("status");
    let text = match self.status {
      Status::Offline    => "OFFLINE",
      Status::Login      => "LOGGING TO OBS...",
      Status::Idle       => "IDLE",
    };
    let widget = Paragraph::new(vec![Spans::from(vec![Span::raw(text)])])
        .block(block);
    f.render_widget(widget, area);
  }
}

