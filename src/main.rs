use std::io;

mod app;
use app::App;
mod status;

fn main() -> Result<(), io::Error> {
  let mut app = App::new()?;
  app.run()?;
  Ok(())
}
