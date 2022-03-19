use sha2::{Digest, Sha256};
use embedded_websocket::{
  WebSocketCloseStatusCode,
  WebSocketSendMessageType,
  WebSocketClient,
  WebSocketOptions,
};
use embedded_websocket::framer::Framer;
use tui::layout::Rect;
use tui::widgets::{Block, Borders, Paragraph};
use tui::text::{Spans, Span};
use tui::backend::Backend;
use tui::Frame;

use std::fs::read_to_string;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use std::thread;

use crate::obs::Status;

use super::AppWidget;

pub struct StatusBar {
  status: Arc<RwLock<Status>>
}

impl StatusBar {
  pub fn new() -> Self {
    let self_status = Arc::new(RwLock::new(Status::Offline));
    let status = self_status.clone();
    thread::spawn(move || {
      let mut stream = loop {
        if let Ok(stream) = TcpStream::connect("localhost:4444") {
          break stream;
        }
        thread::sleep(std::time::Duration::from_millis(500));
      };
      *status.write().unwrap() = Status::Connecting;
      let mut read_buf    = [0; 4000];
      let mut write_buf   = [0; 4000];
      let mut frame_buf   = [0; 4000];
      let mut read_cursor = 0;
      let mut client      = WebSocketClient::new_client(rand::thread_rng());
      let websocket_options = WebSocketOptions {
        path: "/",
        host: "localhost",
        origin: "http://localhost:4444",
        sub_protocols: None,
        additional_headers: None,
      };
      let mut framer = Framer::new(
        &mut read_buf,
        &mut read_cursor,
        &mut write_buf,
        &mut client,
      );
      framer.connect(&mut stream, &websocket_options).unwrap();
      *status.write().unwrap() = Status::Login;
      let message = "{\"message-id\":\"1\",\"request-type\":\"GetAuthRequired\"}";
      framer.write(
        &mut stream,
        WebSocketSendMessageType::Text,
        true,
        message.as_bytes(),
      ).unwrap();

      let mut msg = String::new();
      let get_auth_required: serde_json::Value = loop {
        if let Some(s) = framer.read_text(&mut stream, &mut frame_buf).unwrap() {
          msg.push_str(s);
          if let Ok(get_auth_required) = serde_json::from_str(&msg) {
            break get_auth_required;
          }
        } else {
          panic!("prematory end");
        }
      };
      let mut password = read_to_string("password").unwrap();
      password = password.trim().to_string();
      password = create_auth_response(
        get_auth_required["challenge"].as_str().unwrap(),
        get_auth_required["salt"].as_str().unwrap(),
        &password,
      );
      let message = format!(
        "{{\"message-id\":\"2\",\"request-type\":\"Authenticate\",\"auth\":\"{}\"}}",
        password
      );
      framer.write(
        &mut stream,
        WebSocketSendMessageType::Text,
        true,
        message.as_bytes(),
      ).unwrap();
      while let Some(_s) = framer.read_text(&mut stream, &mut frame_buf).unwrap() {
        framer.close(&mut stream, WebSocketCloseStatusCode::NormalClosure, None).unwrap();
      }
      *status.write().unwrap() = Status::Idle;
    });
    Self{ status: self_status }
  }
}

impl AppWidget for StatusBar {
  fn draw<T: Backend>(&mut self, f: &mut Frame<T>, area: Rect) {
    let block = Block::default()
      .borders(Borders::ALL)
      .title("status");
    let text = match *self.status.read().unwrap() {
      Status::Offline    => "OFFLINE",
      Status::Connecting => "CONNECTING...",
      Status::Login      => "LOGGING TO OBS...",
      Status::Idle       => "IDLE",
    };
    let widget = Paragraph::new(vec![Spans::from(vec![Span::raw(text)])])
        .block(block);
    f.render_widget(widget, area);
  }
}

