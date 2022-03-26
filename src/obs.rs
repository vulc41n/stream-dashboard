use sha2::{Digest, Sha256};
use embedded_websocket::{
  Client,
  WebSocketSendMessageType,
  WebSocketClient,
  WebSocketOptions,
};
use embedded_websocket::framer::Framer;
use rand::rngs::ThreadRng;
use serde_json::Value;

use std::fs::read_to_string;
use std::net::TcpStream;
use std::sync::mpsc::Sender;
use std::thread;

pub struct Obs {
  message_id: u64,
  tx:         Sender<Status>,
}

impl Obs {
  pub fn new(tx: Sender<Status>) -> Self {
    Self{ message_id:  1, tx }
  }

  pub fn run(&mut self) {
    let mut frame_buf   = [0; 4096];
    let mut read_buf    = [0; 4096];
    let mut write_buf   = [0; 4096];
    let mut read_cursor = 0;
    let mut client      = WebSocketClient::new_client(rand::thread_rng());
    let mut connection = self.connect(
      &mut read_buf,
      &mut read_cursor,
      &mut write_buf,
      &mut client,
    );
    self.tx.send(Status::Login).unwrap();
    self.write(&mut connection, "GetAuthRequired", "");
    let get_auth_required = self.read(&mut connection, &mut frame_buf);
    let password = Self::get_password(
      get_auth_required["challenge"].as_str().unwrap(),
      get_auth_required["salt"].as_str().unwrap(),
    );
    self.write(
      &mut connection, "Authenticate", &format!("\"auth\":\"{}\"", password),
    );
    let _response = self.read(&mut connection, &mut frame_buf);
    // println!("{}", response);
    self.tx.send(Status::Idle).unwrap();
    // TODO:
  }

  pub fn connect<'a>(
    &self,
    read_buf:    &'a mut [u8; 4096],
    read_cursor: &'a mut usize,
    write_buf:   &'a mut [u8; 4096],
    client:      &'a mut WebSocketClient<ThreadRng>,
  ) -> Connection<'a> {
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
    let mut framer = Framer::new(read_buf, read_cursor, write_buf, client);
    framer.connect(&mut stream, &websocket_options).unwrap();
    (framer, stream)
  }

  pub fn write(
    &mut self,
    connection: &mut Connection,
    type_: &str,
    msg: &str,
  ) {
    let full_msg = self.get_full_msg(type_, msg);
    let (framer, stream) = connection;
    framer.write(
      stream,
      WebSocketSendMessageType::Text,
      true,
      full_msg.as_bytes(),
    ).unwrap();
  }

  pub fn read(
    &self,
    connection: &mut Connection,
    buf: &mut [u8;4096],
  ) -> Value {
    let (framer, stream) = connection;
    let mut msg = String::new();
    loop {
      if let Some(s) = framer.read_text(stream, buf).unwrap() {
        msg.push_str(s);
        if let Ok(result) = serde_json::from_str(&msg) {
          break result;
        }
      } else {
        panic!("prematory end");
      }
    }
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

  fn get_password(
    challenge: &str,
    salt: &str,
  ) -> String {
    let mut password = read_to_string("password").unwrap();
    password = password.trim().to_string();
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    hasher.update(salt.as_bytes());

    let mut auth = String::with_capacity(Sha256::output_size() * 4 / 3 + 4);

    base64::encode_config_buf(
      hasher.finalize_reset(), base64::STANDARD, &mut auth,
    );

    hasher.update(auth.as_bytes());
    hasher.update(challenge.as_bytes());
    auth.clear();

    base64::encode_config_buf(hasher.finalize(), base64::STANDARD, &mut auth);

    auth
  }
}

pub enum Status {
  Offline,
  Login,
  Idle,
}

type Connection<'a> = (Framer<'a, ThreadRng, Client>, TcpStream);
