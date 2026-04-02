//! Full-screen terminal UI for `passgen` using [Ratatui](https://github.com/ratatui/ratatui).
//!
//! Entry point: [`run`], invoked when the user passes `--tui` / `-t` on the CLI.

mod app;
mod ui;

use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyEventKind};
use ratatui::DefaultTerminal;

/// Initializes the terminal, runs the main loop, and restores the terminal on exit.
pub fn run() -> io::Result<()> {
    let mut terminal = ratatui::try_init()?;
    let result = run_inner(&mut terminal);
    let _ = ratatui::try_restore();
    result
}

fn run_inner(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut app = app::App::new();

    loop {
        terminal.draw(|frame| {
            ui::draw(frame, &mut app);
        })?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && app.handle_key(&key) {
                    break;
                }
            }
        }
    }
    Ok(())
}
