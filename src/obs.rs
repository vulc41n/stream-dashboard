use sha2::{Digest, Sha256};
use embedded_websocket::{
  Client,
  WebSocketCloseStatusCode,
  WebSocketSendMessageType,
  WebSocketClient,
  WebSocketOptions,
};
use embedded_websocket::framer::Framer;
use rand::rngs::ThreadRng;
use tui::layout::Rect;
use tui::widgets::{Block, Borders, Paragraph};
use tui::text::{Spans, Span};
use tui::backend::Backend;
use tui::Frame;

use std::fs::read_to_string;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use std::thread;

pub struct Obs {
  client:      WebSocketClient<ThreadRng>,
  message_id:  u64,
  read_buf:    [u8; 4000],
  write_buf:   [u8; 4000],
  read_cursor: usize,
}

impl Obs {
  pub fn new() -> Self {
    Self{
      client:      WebSocketClient::new_client(rand::thread_rng()),
      message_id:  1,
      read_buf:    [0; 4000],
      write_buf:   [0; 4000],
      read_cursor: 0,
    }
  }

  pub fn run(&mut self) {
    self.connect();
    self.write("GetAuthRequired", "");
    todo!();
  }

  pub fn connect(&mut self) -> Framer<ThreadRng, Client> {
    let mut stream = loop {
      if let Ok(stream) = TcpStream::connect("localhost:4444") {
        break stream;
      }
      thread::sleep(std::time::Duration::from_millis(500));
    };
    // TODO *status.write().unwrap() = Status::Connecting;
    let websocket_options = WebSocketOptions {
      path: "/",
      host: "localhost",
      origin: "http://localhost:4444",
      sub_protocols: None,
      additional_headers: None,
    };
    let mut framer = Framer::new(
      &mut self.read_buf,
      &mut self.read_cursor,
      &mut self.write_buf,
      &mut self.client,
    );
    framer.connect(&mut stream, &websocket_options).unwrap();
    framer
  }

  pub fn write(&mut self, type_: &str, msg: &str) {
    let full_msg = self.get_full_msg(type_, msg);
    todo!();
  }

  fn get_full_msg(&mut self, type_: &str, msg: &str) -> String {
    let mut result = format!(
      "{{\"message-id\":{},\"request-type\":\"{}\"",
      self.message_id, type_,
    ).to_string();
    if !msg.is_empty() {
      result.push(',');
      result.push_str(msg);
    }
    result.push('}');
    self.message_id += 1;
    result
  }

  fn create_auth_response(challenge: &str, salt: &str, password: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(salt.as_bytes());

    let mut auth = String::with_capacity(Sha256::output_size() * 4 / 3 + 4);

    base64::encode_config_buf(hasher.finalize_reset(), base64::STANDARD, &mut auth);

    hasher.update(auth.as_bytes());
    hasher.update(challenge.as_bytes());
    auth.clear();

    base64::encode_config_buf(hasher.finalize(), base64::STANDARD, &mut auth);

    auth
  }
}

pub enum Status {
  Offline,
  Connecting,
  Login,
  Idle,
}
