use tui::layout::{Constraint, Direction, Layout};

use std::io;

use crate::AppWidget;

use super::App;

impl App {
  pub fn render(&mut self) -> Result<(), io::Error> {
    self.terminal.draw(|f| {
      let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
          Constraint::Ratio(2, 2),
          // Constraint::Ratio(1, 2),
        ].as_ref())
        .split(f.size());
      // self.status.draw(f, chunks[0]);
      self.jukebox.draw(f, chunks[0]);
    })?;
    Ok(())
  }
}
