use std::io;
use std::sync::{Arc, Mutex};

use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders};

use crossterm::event::{
  self,
  DisableMouseCapture,
  EnableMouseCapture,
  Event,
  KeyCode,
};
use crossterm::execute;
use crossterm::terminal::{
  disable_raw_mode,
  enable_raw_mode,
  EnterAlternateScreen,
  LeaveAlternateScreen,
};

mod status;
mod widget;

fn main() -> Result<(), io::Error> {
  // setup terminal
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let current = Arc::new(Mutex::new(1));
  loop {
    let mut current = current.lock().unwrap();
    terminal.draw(|f| {
      let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
          Constraint::Percentage(10),
          Constraint::Percentage(80),
          Constraint::Percentage(10)
        ].as_ref())
        .split(f.size());

      let mut block1 = Block::default()
        .borders(Borders::ALL);
      if *current == 1 {
        block1 = block1.border_style(
          Style::default()
          .fg(Color::Green)
          .add_modifier(Modifier::BOLD)
        );
      }
      f.render_widget(block1, chunks[0]);
      let mut block2 = Block::default()
        .title("Block 2")
        .borders(Borders::ALL);
      if *current == 2 {
        block2 = block2.border_style(
          Style::default()
          .fg(Color::Green)
          .add_modifier(Modifier::BOLD)
        );
      }
      f.render_widget(block2, chunks[1]);
    })?;

    match event::read()? {
      Event::Key(event) => {
        if let KeyCode::Char(c) = event.code {
          match c {
            'q' => { break }
            'j' => {
              *current = 2;
            }
            _ => {}
          }
        }
      }
      _ => {}
    }
  }

  // restore terminal
  disable_raw_mode()?;
  execute!(
    terminal.backend_mut(),
    LeaveAlternateScreen,
    DisableMouseCapture
  )?;
  terminal.show_cursor()?;

  Ok(())
}
