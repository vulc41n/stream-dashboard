use crate::AppWidget;

use std::cmp::{max, min};
use std::sync::Arc;
use std::sync::mpsc::{channel, Sender};
use std::thread;

use tui::backend::Backend;
use tui::layout::Rect;
use tui::terminal::Frame;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};
use vlc::{
  Instance,
  Media,
  MediaPlayer,
  MediaPlayerAudioEx,
  Event,
  EventType,
  State,
};

pub struct Player {
  tx:     Sender<Arc<dyn Command>>,
  volume: i32,
}

impl Player {
  pub fn new() -> Self {
    let (tx, commands_rx) = channel::<Arc<dyn Command>>();
    thread::spawn(move || {
      let instance = Instance::new().unwrap();
      let mut filepath = home::home_dir().unwrap();
      filepath.push("music");
      filepath.push("output.ogg");
      let md = Media::new_path(&instance, filepath).unwrap();
      let mdp = MediaPlayer::new(&instance).unwrap();
      let (tx, end_rx) = channel::<()>();
      let em = md.event_manager();
      let _ = em.attach(EventType::MediaStateChanged, move |e, _| {
        match e {
          Event::MediaStateChanged(s) => {
            if s == State::Ended || s == State::Error {
              tx.send(()).unwrap();
            }
          },
          _ => (),
        }
      });
      mdp.set_media(&md);
      mdp.play().unwrap();
      thread::sleep(std::time::Duration::from_millis(200));
      mdp.set_volume(100).unwrap();
      loop {
        if let Ok(_) = end_rx.try_recv() {
          break;
        }
        if let Ok(command) = commands_rx.try_recv() {
          command.run(&mdp);
        }
        thread::sleep(std::time::Duration::from_millis(200));
      }
    });

    Self{
      tx,
      volume: 100,
    }
  }

  pub fn increase_volume(&mut self) {
    self.volume = min(self.volume + 5, 150);
    self.tx.send(Arc::new(Volume(self.volume))).unwrap();
  }

  pub fn decrease_volume(&mut self) {
    self.volume = max(self.volume - 5, 0);
    self.tx.send(Arc::new(Volume(self.volume))).unwrap();
  }
}

impl AppWidget for Player {
  fn draw<T: Backend>(&self, f: &mut Frame<T>, area: Rect) {
    let block = Block::default()
      .borders(Borders::ALL)
      .title("player");
    let text = format!("{}%", self.volume);
    let widget = Paragraph::new(vec![Spans::from(vec![Span::raw(text)])])
        .block(block);
    f.render_widget(widget, area);
  }
}

pub trait Command: Send + Sync {
  fn run(&self, mdp: &MediaPlayer);
}

pub struct Volume(i32);

impl Command for Volume {
  fn run(&self, mdp: &MediaPlayer) {
    mdp.set_volume(self.0).unwrap(); // TODO: print errors
  }
}

unsafe impl Send for Volume {}
unsafe impl Sync for Volume {}
