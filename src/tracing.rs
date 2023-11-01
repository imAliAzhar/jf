use color_eyre::Result;
pub use tracing::*;

pub fn init() -> Result<()> {
    let file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("/tmp/jf/log")?;

    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_writer(file)
        .with_target(false)
        .with_ansi(true)
        .with_max_level(Level::DEBUG)
        .init();

    tracing::trace!("Tracing initialized");

    Ok(())
}
