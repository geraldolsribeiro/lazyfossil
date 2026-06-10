mod app;
mod config;
mod fossil;
mod ui;

use anyhow::Result;

fn main() -> Result<()> {
    let debug_enabled = std::env::args().any(|arg| arg == "--debug");
    app::run(debug_enabled)
}
