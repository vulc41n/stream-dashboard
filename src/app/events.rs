use crossterm::event::{Event, poll, KeyCode, read};

use std::io;
use std::time::Duration;

use crate::player::Command;
use super::App;

impl App {
  pub fn handle_events(&self) -> Result<bool, io::Error> {
    let mut state = self.state.lock().unwrap();
    if let Ok(true) = poll(Duration::from_millis(200)) {
      match read()? {
        Event::Key(event) => {
          if let KeyCode::Char(c) = event.code {
            match c {
              'q' => { return Ok(true) }
              'j' => { state.current_y += 1; }
              'k' => { state.current_y -= 1; }
              'h' => { /* state.current_x += 1; */ }
              'l' => { /* state.current_x -= 1; */ }
              '-' => { state.tx_player.send(Command::Volume(-0.05)).unwrap(); }
              '=' => { state.tx_player.send(Command::Volume(0.05)).unwrap(); }
              '>' => { state.tx_player.send(Command::SongControl(1)).unwrap(); }
              '<' => { state.tx_player.send(Command::SongControl(-1)).unwrap(); }
              ' ' => { state.tx_player.send(Command::TogglePause).unwrap(); }
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
