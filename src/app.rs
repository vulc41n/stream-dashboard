use crate::AppWidget;
use crate::player::Player;
use crate::status::StatusBar;

use std::io;
use std::time::Duration;
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
        current_y: 1,
        // current_x: 1,
        player: Player::new(),
        status: StatusBar::new(),
      })),
    })
  }

  pub fn run(&mut self) -> Result<(), io::Error> {
    loop {
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
        state.player.draw(f, chunks[1]);
      })?;

      if let Ok(true) = event::poll(Duration::from_millis(200)) {
        match event::read()? {
          Event::Key(event) => {
            if let KeyCode::Char(c) = event.code {
              match c {
                'q' => { break }
                'j' => { state.current_y += 1; }
                'k' => { state.current_y -= 1; }
                'h' => { /* state.current_x += 1; */ }
                'l' => { /* state.current_x -= 1; */ }
                '-' => { state.player.decrease_volume(); }
                '=' => { state.player.increase_volume(); }
                '>' => { state.player.next_song(); }
                '<' => { state.player.previous_song(); }
                ' ' => { state.player.toggle_pause(); }
                _ => {}
              }
            }
          }
          _ => {}
        }
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
  // pub current_x: u8,
  pub current_y: u8,
  pub player:    Player,
  pub status:    StatusBar,
}

