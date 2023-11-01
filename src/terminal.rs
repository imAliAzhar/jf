use color_eyre::Result;
use std::io::Stdout;
use tracing::debug;

use ratatui::prelude::CrosstermBackend;

pub struct Terminal {
    terminal: ratatui::Terminal<CrosstermBackend<Stdout>>,
}

pub type Frame<'a> = ratatui::Frame<'a>;

impl Terminal {
    pub fn init() -> Result<Self> {
        initialize_panic_handler();

        // Startup
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stderr(), crossterm::terminal::EnterAlternateScreen)?;

        let terminal = ratatui::Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

        Ok(Self { terminal })
    }

    pub fn draw<F>(&mut self, f: F) -> Result<()>
    where
        F: FnOnce(&mut Frame),
    {
        debug!("Rendering new frame...");
        self.terminal.draw(f)?;
        Ok(())
    }
}

fn initialize_panic_handler() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen).unwrap();
        crossterm::terminal::disable_raw_mode().unwrap();
        original_hook(panic_info);
    }));
}

impl Drop for Terminal {
    fn drop(&mut self) {
        crossterm::execute!(std::io::stderr(), crossterm::terminal::LeaveAlternateScreen).ok();
        crossterm::terminal::disable_raw_mode().ok();
    }
}
