use id3::{Tag, TagLike};
use rand::thread_rng;
use rand::seq::SliceRandom;
use rodio::{OutputStream, OutputStreamHandle};

use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

mod command;
pub use command::Command;

pub struct Player {
  current:  usize,
  credits:  String,
  handle:   OutputStreamHandle,
  playlist: Vec<PathBuf>,
  rx:       Receiver<Command>,
  #[allow(dead_code)] // steam must be alive to play songs
  stream:   OutputStream,
  tx:       Sender<String>,
  volume:   f32,
}

impl Player {
  pub fn new(
    path: PathBuf,
    rx:       Receiver<Command>,
    tx:       Sender<String>,
    volume:   f32,
  ) -> Self {
    let (stream, handle) = OutputStream::try_default().unwrap();
    let mut playlist = Vec::new();
    for file in path.read_dir().unwrap() {
      if let Ok(file) = file {
        if file.path().extension().unwrap() == "mp3" {
          playlist.push(file.path());
        }
      }
    }
    playlist.shuffle(&mut thread_rng());
    Self{
      current: 0,
      credits: String::new(),
      playlist, handle, rx, stream, tx, volume,
    }
  }

  pub fn run(&mut self) {
    loop {
      if self.current >= self.playlist.len() {
        self.current = 0;
        self.playlist.shuffle(&mut thread_rng());
      }
      let file = self.set_credits();
      let br = BufReader::new(File::open(file).unwrap());
      let sink = self.handle.play_once(br).unwrap();
      sink.set_volume(self.volume);
      loop {
        if let Ok(command) = self.rx.recv_timeout(
          Duration::from_millis(500)
        ) {
          if command.run(self, &sink) {
            break;
          }
        }
        if sink.empty() {
          self.current += 1;
          break; // next song
        }
      }
    }
  }

  pub fn set_volume(&mut self, offset: f32) {
    self.volume += offset;
    if self.volume < 0.0 {
      self.volume = 0.0;
    } else if self.volume > 1.0 {
      self.volume = 1.0;
    }
    self.send_display();
  }

  fn set_credits(&mut self) -> &PathBuf {
    let file = &self.playlist[self.current];
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
    self.credits = format!("{} - {} {}", title, artist, comments).to_string();
    self.send_display();
    file
  }

  fn send_display(&self) {
    let volume = (self.volume*100.0).round() as u8;
    self.tx.send(format!("{} | {}%", self.credits, volume)).unwrap();
  }
}

unsafe impl Send for Player {}
unsafe impl Sync for Player {}
