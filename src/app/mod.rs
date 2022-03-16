use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::execute;
use crossterm::terminal::{
  disable_raw_mode,
  enable_raw_mode,
  EnterAlternateScreen,
  LeaveAlternateScreen,
};
use tui::Terminal;
use tui::backend::CrosstermBackend;

use std::io;
use std::sync::{Arc, Mutex};

use crate::widgets::{AppWidget, Jukebox, StatusBar};

mod events;
mod render;
mod state;
use state::State;

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
      state:   Arc::new(Mutex::new(State{
        current_y: 1,
        // current_x: 1,
        jukebox: Jukebox::new(),
        status:  StatusBar::new(),
      })),
    })
  }

  pub fn run(&mut self) -> Result<(), io::Error> {
    loop {
      self.render()?;
      if self.handle_events()? {
        break
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

