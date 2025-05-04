use std::env;
use std::path::PathBuf;
use std::error::Error;

mod app;
mod ui;
mod log_parser;

use app::LogViewer;

fn main() -> Result<(), Box<dyn Error>> {
    let directory = env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| PathBuf::from("."));
    
    let mut app = LogViewer::new(directory)?;
    app.run()?;
    
    Ok(())
}
