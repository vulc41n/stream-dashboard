use std::thread;
use vlc::{Instance, Media, MediaPlayer};

pub struct Player {
}

impl Player {
  pub fn new() -> Self {
    // Start playing
    thread::spawn(|| {
      // Create an instance
      let instance = Instance::new().unwrap();
      // Create a media from a file
      let mut filepath = home::home_dir().unwrap();
      filepath.push("music");
      filepath.push("output.ogg");
      let md = Media::new_path(&instance, filepath).unwrap();
      // Create a media player
      let mdp = MediaPlayer::new(&instance).unwrap();
      mdp.set_media(&md);
      mdp.play().unwrap();
      thread::sleep(std::time::Duration::from_secs(10));
    });

    Self{}
  }
}
