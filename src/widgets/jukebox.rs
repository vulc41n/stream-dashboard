use tui::backend::Backend;
use tui::layout::Rect;
use tui::terminal::Frame;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};

use std::sync::mpsc::Receiver;

use super::AppWidget;

pub struct Jukebox {
  display: String,
  rx:      Receiver<String>,
}

impl Jukebox {
  pub fn new(rx: Receiver<String>) -> Self {
    Self{
      display: String::new(),
      rx,
    }
  }
}

impl AppWidget for Jukebox {
  fn draw<T: Backend>(&mut self, f: &mut Frame<T>, area: Rect) {
    if let Ok(display) = self.rx.try_recv() {
      self.display = display;
    }
    let block = Block::default()
      .borders(Borders::ALL)
      .title("jukebox");
    let text = self.display.clone();
    let widget = Paragraph::new(vec![Spans::from(vec![Span::raw(text)])])
        .block(block);
    f.render_widget(widget, area);
  }
}

