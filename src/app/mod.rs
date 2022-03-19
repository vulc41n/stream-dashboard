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
use std::sync::mpsc::channel;
use std::thread::spawn;

use crate::player::{Command, Player};
use crate::widgets::{AppWidget, Jukebox, StatusBar};

mod events;
mod render;
mod state;
use state::State;

pub struct App {
  jukebox:  Jukebox,
  player:   Arc<Mutex<Player>>,
  state:    Arc<Mutex<State>>,
  status:   StatusBar,
  terminal: Terminal<CrosstermBackend<io::Stdout>>,
}

impl App {
  pub fn new() -> Result<Self, io::Error> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;

    // TODO: setup player
    let mut dirpath = home::home_dir().unwrap();
    dirpath.push("music");
    let (tx_player, rx_player) = channel::<Command>();
    let (tx_display, rx_display) = channel::<String>();

    Ok(Self{
      jukebox: Jukebox::new(rx_display),
      player:  Arc::new(Mutex::new(
        Player::new(dirpath, rx_player, tx_display, 0.1)
      )),
      state:   Arc::new(Mutex::new(State{
        current_y: 1,
        // current_x: 1,
        tx_player,
      })),
      status:  StatusBar::new(),
      terminal,
    })
  }

  pub fn run(&mut self) -> Result<(), io::Error> {
    {
      let player = self.player.clone();
      spawn(move || {
        player.lock().unwrap().run();
      });
    }
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

