use super::App;

use crate::ui::render;
use std::time::{Duration, Instant};
use crossterm::event::{self, Event, KeyCode};

use ratatui::DefaultTerminal;

pub fn falling_block(mut app: &mut App, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(1000);
    while !app.falling {
        terminal.draw(|f| render(&mut app, f))?;

        app.erase_fall_block();

        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => {
                        app.loss();
                    },
                    KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Left => app.left(),
                    KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down => {app.down(); last_tick = Instant::now();},
                    KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Right => app.right(),
                    KeyCode::Char(' ') => app.space_bar(),
                    // вращение
                    KeyCode::Char('q') | KeyCode::Char('Q') => app.spin_left(),
                    KeyCode::Char('e') | KeyCode::Char('E') => app.spin_right(),
                    // тесты
                    // KeyCode::Char('t') | KeyCode::Char('T') => app.full_row(),

                    _ => {}
                }
            }
        }

        // падает блок через каждые 500мс
        if last_tick.elapsed() >= tick_rate {
            app.down();
            last_tick = Instant::now();
        }

        // рисую падающий блок
        app.paint_fall_block();
    }
    // после падения блок нарисовать
    app.paint_fall_block();
    Ok(())
}

impl App {
    pub fn paint_fall_block(&mut self) {
        for [x, y] in self.fall_block.iter() {
            self.matrix[*x as usize][*y as usize] = self.color_block;
        }
    }

    pub fn erase_fall_block(&mut self) {
        for [x, y] in self.fall_block.iter() {
            self.matrix[*x as usize][*y as usize] = 0;
        }
    }

    pub fn possible_action(&mut self, axis: u8, run: i8) -> bool {
        let max_bound  = if axis == 0 { self.matrix.len() - 1 } else if run < 0 {0} else { self.matrix[0].len() - 1 };
        for row in 0..self.fall_block.len() {
            if self.fall_block[row][axis as usize] == max_bound as u8 {
                return false;
            }
            let y = if axis == 0 {((self.fall_block[row][0]) as i8 + run) as usize} else {self.fall_block[row][0] as usize};
            let x = if axis == 0 {(self.fall_block[row][1]) as usize} else {((self.fall_block[row][1]) as i8 + run) as usize};

            if self.matrix[y][x] != 0 {
                return false;
            }

        }
        true
    }

    pub fn possible_left(&mut self) -> bool {
        self.possible_action(1, -1)
    }

    pub fn possible_right(&mut self) -> bool {
        self.possible_action(1, 1)
    }

    pub fn block_falling(&mut self) -> bool {
        self.possible_action(0, 1)
    }

    //move

    pub fn rotation(&mut self, spin: i8) {
        let center_y = self.fall_block[1][0] as i8;
        let center_x = self.fall_block[1][1] as i8;

        let old_turn = self.turn;

        self.turn = self.turn + spin;
        if self.turn > 3 {
            self.turn = 0;
        } else if self.turn < 0 {
            self.turn = 3;
        }

        let mut new_positions = [[0u8; 2]; 4];
        let mut can_rotate = true;

        let block_rotation = self.current_figure[self.turn as usize];

        for (i, &(x, y)) in block_rotation.iter().enumerate() {
            let y1 = center_y + y;
            let x1 = center_x + x;

            if y1 < 0 || y1 >= 20 || x1 < 0 || x1 >= 10 {
                can_rotate = false;
                break;
            }

            if self.matrix[y1 as usize][x1 as usize] != 0 {
                can_rotate = false;
                break;
            }

            new_positions[i] = [y1 as u8, x1 as u8];
        }

        if can_rotate {
            self.fall_block = new_positions;
        } else {
            self.turn = old_turn;
        }
    }

    pub fn spin_left(&mut self) {
        self.rotation(1);
    }

    pub fn spin_right(&mut self) {
        self.rotation(-1);
    }

    pub fn handling_move(&mut self, axis:u8 , run: i8) {
        for i in 0..self.fall_block.len() {
            self.fall_block[i][axis as usize] = ((self.fall_block[i][axis as usize]) as i8 + run) as u8
        }
    }

    pub fn down(&mut self) {
        if self.block_falling() {
            self.handling_move(0, 1)
        } else {
            self.falling = true;
        }
    }

    pub fn left(&mut self) {
        if self.possible_left() {
            self.handling_move(1, -1)
        }
    }

    pub fn right(&mut self) {
        if self.possible_right() {
            self.handling_move(1, 1)
        }
    }

    pub fn up(&mut self) {
        self.handling_move(0, -1)
    }

    pub fn space_bar(&mut self) {
        while self.block_falling() {
            self.handling_move(0, 1);
        }
        self.falling = true;
    }


}
