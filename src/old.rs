use crate::widgets::{AppWidget, Jukebox, StatusBar};

use std::io;
use std::time::Duration;
use std::sync::{Arc, LockResult, Mutex, MutexGuard};

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

  fn render(&mut self) -> Result<(), io::Error> {
    let mut state = self.state.lock().unwrap();
    self.terminal.draw(|f| {
      let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
          Constraint::Ratio(1, 2),
          Constraint::Ratio(1, 2),
        ].as_ref())
        .split(f.size());
      state.status.draw(f, chunks[0]);
      state.jukebox.draw(f, chunks[1]);
    })?;
    Ok(())
  }

  fn handle_events(&self) -> Result<bool, io::Error> {
    let mut state = self.state.lock().unwrap();
    if let Ok(true) = event::poll(Duration::from_millis(200)) {
      match event::read()? {
        Event::Key(event) => {
          if let KeyCode::Char(c) = event.code {
            match c {
              'q' => { return Ok(true) }
              'j' => { state.current_y += 1; }
              'k' => { state.current_y -= 1; }
              'h' => { /* state.current_x += 1; */ }
              'l' => { /* state.current_x -= 1; */ }
              '-' => { state.jukebox.decrease_volume(); }
              '=' => { state.jukebox.increase_volume(); }
              '>' => { state.jukebox.next_song(); }
              '<' => { state.jukebox.previous_song(); }
              ' ' => { state.jukebox.toggle_pause(); }
              _ => {}
            }
          }
        }
        _ => {}
      }
    }
    Ok(false)
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
  // pub current_x: u8,
  pub current_y: u8,
  pub jukebox:   Jukebox,
  pub status:    StatusBar,
}

