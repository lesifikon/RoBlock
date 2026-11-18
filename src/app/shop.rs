use super::App;


use crate::ui::render;
use std::time::{Duration};
use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;


pub fn shop(mut app: &mut App, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {

    while !app.config.shop {
        terminal.draw(|f| render(&mut app, f))?;

        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => {
                        app.loss();
                    },
                    KeyCode::Char('q') | KeyCode::Char('Q') => app.leave_shop(),
                    _ => {}
                }
            }
        }
    }
    app.hendler_leave();
    Ok(())

}

impl App {
    pub fn leave_shop(&mut self) {
        self.config.shop = true;
    }

    pub fn hendler_leave(&mut self) {
        self.game = false;
        self.config.shop = false
    }

}
