use std::io;

mod app;
use app::App;
mod obs;
mod player;
mod widgets;
use widgets::AppWidget;

fn main() -> Result<(), io::Error> {
  let mut app = App::new()?;
  app.run()?;
  Ok(())
}
