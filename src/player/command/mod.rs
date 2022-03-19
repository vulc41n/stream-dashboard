use rodio::Sink;

use super::Player;

pub enum Command {
  Volume(f32),
  SongControl(i32),
  TogglePause,
}

unsafe impl Send for Command {}
unsafe impl Sync for Command {}

impl Command {
  pub fn run(&self, player: &mut Player, sink: &Sink) -> bool {
    match self {
      &Command::Volume(offset) => {
        player.set_volume(offset);
        sink.set_volume(player.volume);
      }
      &Command::SongControl(offset) => {
        if offset != 0 {
          if offset > 0 {
            player.current += offset as usize;
          } else {
            player.current -= offset.abs() as usize;
          }
          return true;
        }
      }
      &Command::TogglePause => {
        if sink.is_paused () {
          sink.play();
        } else {
          sink.pause();
        }
      }
    }
    false
  }
}
