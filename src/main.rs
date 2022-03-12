use std::io;

mod app;
use app::App;
mod player;
// mod status;
mod widget;
use widget::AppWidget;

fn main() -> Result<(), io::Error> {
  let mut app = App::new()?;
  app.run()?;
  Ok(())
}
