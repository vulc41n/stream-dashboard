use std::io;
use std::sync::{Arc, Mutex};

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

use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders};

pub struct App {
  terminal: Terminal<CrosstermBackend<io::Stdout>>,
  state: Arc<Mutex<State>>,
}

impl App {
  pub fn new() -> Result<Self, io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    Ok(Self{
      terminal,
      state: Arc::new(Mutex::new(State{
        current: 1,
      })),
    })
  }

  pub fn run(&mut self) -> Result<(), io::Error> {
    loop {
      let mut state = self.state.lock().unwrap();
      self.terminal.draw(|f| {
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
        if state.current == 1 {
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
        if state.current == 2 {
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
              'j' => { state.current += 1; }
              'k' => { state.current -= 1; }
              _ => {}
            }
          }
        }
        _ => {}
      }
    }
    Ok(())
  }
}

impl Drop for App {
  fn drop(&mut self) {
    // restore terminal
    disable_raw_mode().unwrap();
    execute!(
      self.terminal.backend_mut(),
      LeaveAlternateScreen,
      DisableMouseCapture
    ).unwrap();
    self.terminal.show_cursor().unwrap();
  }
}

struct State {
  pub current: u8,
}

