use crate::AppWidget;

use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;

use rand::thread_rng;
use rand::seq::SliceRandom;
use rodio::{OutputStream, Sink};
use tui::backend::Backend;
use tui::layout::Rect;
use tui::terminal::Frame;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};

pub struct Player {
  tx:     Sender<Arc<dyn Command>>,
  volume: f32,
  stream: OutputStream,
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

    let (tx, commands_rx) = channel::<Arc<dyn Command>>();
    thread::spawn(move || {
      let mut current = 0;
      loop {
        let file = &playlist[current];
        let br = BufReader::new(File::open(file).unwrap());
        let sink = stream_handle.play_once(br).unwrap();
        loop {
          if let Ok(command) = commands_rx.recv_timeout(
            Duration::from_millis(200)
          ) {
            let song_control = command.run(&sink);
            if song_control != 0 {
              if song_control > 0 {
                current += song_control as usize;
              } else {
                current -= song_control.abs() as usize;
              }
              break;
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
      tx, stream,
      volume: 1.0,
    }
  }

  pub fn increase_volume(&mut self) {
    let new_volume = self.volume + 0.05;
    self.volume = if new_volume > 1.0 { 1.0 } else { new_volume };
    self.tx.send(Arc::new(Volume(self.volume))).unwrap();
  }

  pub fn decrease_volume(&mut self) {
    let new_volume = self.volume - 0.05;
    self.volume = if new_volume < 0.0 { 0.0 } else { new_volume };
    self.tx.send(Arc::new(Volume(self.volume))).unwrap();
  }

  pub fn next_song(&mut self) {
    self.tx.send(Arc::new(SongControl(1))).unwrap();
  }

  pub fn previous_song(&mut self) {
    self.tx.send(Arc::new(SongControl(-1))).unwrap();
  }
}

impl AppWidget for Player {
  fn draw<T: Backend>(&self, f: &mut Frame<T>, area: Rect) {
    let block = Block::default()
      .borders(Borders::ALL)
      .title("player");
    let text = format!("{}%", (self.volume*100.0).round() as u8);
    let widget = Paragraph::new(vec![Spans::from(vec![Span::raw(text)])])
        .block(block);
    f.render_widget(widget, area);
  }
}

pub trait Command: Send + Sync {
  fn run(&self, sink: &Sink) -> i32;
}

pub struct Volume(f32);

impl Command for Volume {
  fn run(&self, sink: &Sink) -> i32 {
    sink.set_volume(self.0);
    0
  }
}

pub struct SongControl(i32);

impl Command for SongControl {
  fn run(&self, sink: &Sink) -> i32 { self.0 }
}

unsafe impl Send for Volume {}
unsafe impl Sync for Volume {}
