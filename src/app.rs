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
    let tick_rate = Duration::from_millis(1000);

    loop {
        // урисую падающий блок

        terminal.draw(|f| render(&mut app, f))?;

        app.erase_fall_block();

        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        break Ok(());
                    },
                    KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Left => app.left(),
                    KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down => { if app.block_falling() {app.down(); last_tick = Instant::now();}},
                    KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Right => app.right(),
                    _ => {}
                }
            }
        }

        // падает блок через каждые 500мс
        if last_tick.elapsed() >= tick_rate {
            if app.block_falling() {
                app.down();
            } else {
                app.paint_fall_block();
                app.spawn_block();
            }
            last_tick = Instant::now();
        }

        // рисую падающий блок
        app.paint_fall_block();
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

    pub fn block_falling(&mut self) -> bool {
        for row in 0..self.fall_block.len() {
            if self.fall_block[row][0] == (self.matrix.len() - 1) as u8 {
                return false;
            }
            let next_y = (self.fall_block[row][0] + 1) as usize;
            if self.matrix[next_y][self.fall_block[row][1] as usize] != 0 {
                return false;
            }
        }
        true
    }

    //move
    
    pub fn handling_move(&mut self, x:u8 , run: i8) {
        for i in 0..self.fall_block.len() {
            self.fall_block[i][x as usize] = ((self.fall_block[i][x as usize]) as i8 + run) as u8
        }
    }

    pub fn down(&mut self) {
        self.handling_move(0, 1)
    }

    pub fn left(&mut self) {
        self.handling_move(1, -1)
    }

    pub fn right(&mut self) {
        self.handling_move(1, 1)
    }

    pub fn up(&mut self) {
        self.handling_move(0, -1)
    }


}
