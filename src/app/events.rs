use crossterm::event::{Event, poll, KeyCode, read};

use std::io;
use std::time::Duration;

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
