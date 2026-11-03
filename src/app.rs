use crate::ui::render;
use std::time::{Duration, Instant};
use crossterm::event::{self, Event, KeyCode};

use ratatui::DefaultTerminal;

// let matrix = [[0u8; 10]; 20];

pub fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut app = App::new();
    app.spawn_block();
    app.paint_fall_block();

    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(500);

    loop {
        terminal.draw(|f| render(&mut app, f))?;

        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break Ok(());
                    },
                    KeyCode::Char('a') | KeyCode::Left => app.left(),
                    KeyCode::Char('d') | KeyCode::Right => app.right(),
                    _ => {}
                }
            }
        }

        // падает блок через каждые 500мс
        if last_tick.elapsed() >= tick_rate {
            if app.block_falling() {
                app.down();
            } else {
                app.spawn_block();
            }
            last_tick = Instant::now(); 
        }


    }
}

#[derive(Debug, Default)]
pub struct App {
    // карта
    pub matrix: [[u8; 10]; 20],
    // падающий блок
    pub fall_block: [[u8; 2]; 4],
    // очки
    pub glasses: u64,
}

impl App {

    pub fn new() -> Self {
        Self::default()
    }

    pub fn paint_fall_block(&mut self) {
        for [x, y] in self.fall_block.iter() {
            self.matrix[*x as usize][*y as usize] = 1;
        }
    }

    pub fn erase_fall_block(&mut self) {
        for [x, y] in self.fall_block.iter() {
            self.matrix[*x as usize][*y as usize] = 0;
        }
    }

    pub fn spawn_block(&mut self) {
        self.fall_block = [
            [1, 3],
            [1, 4],
            [1, 5],
            [0, 5],
        ];
    }

    pub fn down(&mut self) {
        self.erase_fall_block();
        for i in 0..self.fall_block.len() {
            let number = self.fall_block[i][0];
            self.fall_block[i][0] = number + 1
        }
        self.paint_fall_block();
    }

    pub fn block_falling(&mut self) -> bool {
        self.erase_fall_block();
        for row in 0..self.fall_block.len() {
            if self.fall_block[row][0] == (self.matrix.len() - 1) as u8 {
                self.paint_fall_block();
                return false;
            }
            let next_y = (self.fall_block[row][0] + 1) as usize;
            if self.matrix[next_y][self.fall_block[row][1] as usize] != 0 {
                self.paint_fall_block();
                return false;
            }
        }
        self.paint_fall_block();
        true
    }

    //move
    pub fn left(&mut self) {
        self.erase_fall_block();
        for i in 0..self.fall_block.len() {
            let number = self.fall_block[i][1];
            self.fall_block[i][1] = number - 1
        }
        self.paint_fall_block();
    }

    pub fn right(&mut self) {
        self.erase_fall_block();
        for i in 0..self.fall_block.len() {
            let number = self.fall_block[i][1];
            self.fall_block[i][1] = number + 1
        }
        self.paint_fall_block();
    }


}
