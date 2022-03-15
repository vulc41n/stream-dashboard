use crate::AppWidget;

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::time::Duration;

use id3::{Tag, TagLike};
use rand::thread_rng;
use rand::seq::SliceRandom;
use rodio::{OutputStream, Sink};
use tui::backend::Backend;
use tui::layout::Rect;
use tui::terminal::Frame;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};

pub struct Player {
  tx_commands: Sender<Command>,
  volume:      f32,
  stream:      OutputStream,
  rx_display:  Receiver<String>,
  display:     String,
}

impl Player {
  pub fn new() -> Self {
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut playlist = Vec::new();
    let mut dirpath = home::home_dir().unwrap();
    dirpath.push("music");
    for file in dirpath.read_dir().unwrap() {
      if let Ok(file) = file {
        if file.path().extension().unwrap() == "mp3" {
          playlist.push(file.path());
        }
      }
    }
    playlist.shuffle(&mut thread_rng());

    let (tx_commands, rx_commands) = channel::<Command>();
    let (tx_display, rx_display) = channel::<String>();
    thread::spawn(move || {
      let mut current = 0;
      let mut volume = 1.0;
      loop {
        let file = &playlist[current];
        let tag = Tag::read_from_path(file).unwrap();
        let title = tag.title().unwrap_or("unknown");
        let artist = tag.artist().unwrap_or("unknown");
        let mut comments = String::new();
        for com in tag.comments() {
          if comments.is_empty() {
            comments.push('(');
          } else {
            comments.push_str(", ");
          }
          comments.push_str(&com.to_string());
        }
        if !comments.is_empty() { comments.push(')'); }
        tx_display.send(format!(
            "{} - {} {}", title, artist, comments,
        )).unwrap();
        let br = BufReader::new(File::open(file).unwrap());
        let sink = stream_handle.play_once(br).unwrap();
        sink.set_volume(volume);
        loop {
          if let Ok(command) = rx_commands.recv_timeout(
            Duration::from_millis(200)
          ) {
            match command {
              Command::Volume(v) => {
                volume = v;
                sink.set_volume(v);
              }
              Command::SongControl(offset) => {
                if offset != 0 {
                  if offset > 0 {
                    current += offset as usize;
                  } else {
                    current -= offset.abs() as usize;
                  }
                  break;
                }
              }
              Command::TogglePause => {
                if sink.is_paused () {
                  sink.play();
                } else {
                  sink.pause();
                }
              }
            }
          }
          if sink.empty() {
            current += 1;
            break; // next song
          }
        }
      }
    });

    Self{
      rx_display, tx_commands, stream,
      volume:  1.0,
      display: String::new(),
    }
  }

  pub fn increase_volume(&mut self) {
    let new_volume = self.volume + 0.05;
    self.volume = if new_volume > 1.0 { 1.0 } else { new_volume };
    self.tx_commands.send(Command::Volume(self.volume)).unwrap();
  }

  pub fn decrease_volume(&mut self) {
    let new_volume = self.volume - 0.05;
    self.volume = if new_volume < 0.0 { 0.0 } else { new_volume };
    self.tx_commands.send(Command::Volume(self.volume)).unwrap();
  }

  pub fn next_song(&mut self) {
    self.tx_commands.send(Command::SongControl(1)).unwrap();
  }

  pub fn previous_song(&mut self) {
    self.tx_commands.send(Command::SongControl(-1)).unwrap();
  }

  pub fn toggle_pause(&mut self) {
    self.tx_commands.send(Command::TogglePause).unwrap();
  }
}

impl AppWidget for Player {
  fn draw<T: Backend>(&mut self, f: &mut Frame<T>, area: Rect) {
    if let Ok(display) = self.rx_display.try_recv() {
      self.display = display;
    }
    let block = Block::default()
      .borders(Borders::ALL)
      .title("player");
    let text = format!(
      "{} | {}%",
      self.display,
      (self.volume*100.0).round() as u8,
    );
    let widget = Paragraph::new(vec![Spans::from(vec![Span::raw(text)])])
        .block(block);
    f.render_widget(widget, area);
  }
}

enum Command {
  Volume(f32),
  SongControl(i32),
  TogglePause,
}

unsafe impl Send for Command {}
unsafe impl Sync for Command {}
