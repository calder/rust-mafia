//! Terminal user interface.

use std::io;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders};
use tui::Terminal;

pub struct Client {
    terminal: Terminal<tui::backend::TermionBackend<termion::raw::RawTerminal<io::Stdout>>>,
}

impl Client {
    pub fn new() -> Result<Client, io::Error> {
        let stdout = io::stdout().into_raw_mode()?;
        let backend = TermionBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        Ok(Client { terminal: terminal })
    }

    pub fn draw(self: &mut Self) -> Result<(), io::Error> {
        self.terminal.clear()?;

        self.terminal.draw(|mut f| {
            // Split the screen into 2 columns.
            let view = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(f.size());

            // Split the left column into 2 rows.
            let left = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(70), Constraint::Percentage(30)].as_ref())
                .split(view[0]);

            // Player list.
            let block = Block::default().title("Players").borders(Borders::ALL);
            f.render_widget(block, view[1]);

            // Action picker.
            let block = Block::default().title("Actions").borders(Borders::ALL);
            f.render_widget(block, left[0]);

            // Event log.
            let block = Block::default().title("Events").borders(Borders::ALL);
            f.render_widget(block, left[1]);
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
