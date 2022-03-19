use std::sync::mpsc::Sender;

use crate::player::{Command, Player};

pub struct State {
  // pub current_x: u8,
  pub current_y: u8,
  pub tx_player: Sender<Command>,
}

unsafe impl Sync for State {}
unsafe impl Send for State {}

