mod app;
mod fossil;
mod ui;

use anyhow::Result;

fn main() -> Result<()> {
    app::run()
}
