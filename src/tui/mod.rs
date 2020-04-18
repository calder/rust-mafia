//! Terminal user interface.

use std::io;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders};
use tui::Terminal;

struct App<Backend: tui::backend::Backend> {
    terminal: Terminal<Backend>,
}

impl<Backend: tui::backend::Backend> App<Backend> {
    pub fn new(terminal: Terminal<Backend>) -> App<Backend> {
        App { terminal: terminal }
    }

    pub fn draw(self: &mut Self) -> Result<(), io::Error> {
        self.terminal.clear()?;

        self.terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            let block = Block::default().title("Block").borders(Borders::ALL);
            f.render_widget(block, chunks[0]);
            let block = Block::default().title("Block 2").borders(Borders::ALL);
            f.render_widget(block, chunks[1]);
        })
    }

    pub fn run(self: &mut Self) -> Result<(), io::Error> {
        self.draw()?;
        for c in io::stdin().keys() {
            match c.unwrap() {
                Key::Char('q') => break,
                _ => {}
            }
            self.draw()?;
        }

        Ok(())
    }
}

/// Entry point.
pub fn main() -> Result<(), io::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    let mut app = App::new(terminal);

    app.run()
}
