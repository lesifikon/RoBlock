use crate::ui::render;
use std::time::Duration;
use crossterm::event::{self, Event, KeyCode};

use ratatui::DefaultTerminal;

// let matrix = [[0u8; 10]; 20];

pub fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut app = App::new();
    app.spawn_block();
    app.paint_fall_block();
    // app.down();
    app.check_down();
    loop {
        // terminal.draw(|f| render(&mut app, f))?;
        // if crossterm::event::read()?.is_key_press() {
        //     break Ok(());
        // }
        terminal.draw(|f| render(&mut app, f))?;

        // 2. Проверяем, нажал ли пользователь что-то (ждем максимум 500 мс)
        // Это заменяет thread::sleep и одновременно обрабатывает ввод!
        if event::poll(Duration::from_millis(1000))? {
            // Если событие есть, читаем его (теперь read() сработает мгновенно)
            if let Event::Key(key) = event::read()? {
                // Проверяем конкретную клавишу, например, Esc или 'q' для выхода
                if key.code == KeyCode::Esc || key.code == KeyCode::Char('q') {
                    break Ok(());
                }
 
            }
        }

        // 3. Логика игры (срабатывает каждые ~500 мс, если не было нажатий, 
        // либо сразу после обработки нажатия клавиши)
        if  app.matrix[19][5] != 1 && app.fall_block[2][0] != 19{
            app.down();
            app.left();
        } else {
            app.spawn_block();
            app.paint_fall_block();
        }
        app.check_down();


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

    pub fn erase_fall_block_left(&mut self) {
        for [x, y] in self.fall_block.iter() {
            self.matrix[*x as usize][*y as usize + 1] = 0;
        }
    }

    // pub fn erase_fall_block(&mut self) {
    //     for [x, y] in self.fall_block.iter() {
    //         self.matrix[*x as usize][*y as usize] = 0;
    //     }
    // }


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

    pub fn check_down(&mut self) {
        // for i in 0..self.matrix.len() {
        //     for j in 0..self.matrix[i].len() {
        //         if i == 19 {
        //             // блок коснулся дна
        //             togo!();
        //         }
        //         for [x, y] in self.fall_block.iter() {
        //             if 
        //         }
        //     }
        // }
        for row in 0..self.fall_block.len() {
            if self.fall_block[row][0] == (self.matrix.len() - 1) as u8 {
                println!("упаф");
            }
            // проверить не коснулся ли блок стоячей блок
        }
    }

    //move
    pub fn left(&mut self) {
        self.erase_fall_block_left();
        for i in 0..self.fall_block.len() {
            let number = self.fall_block[i][1];
            self.fall_block[i][1] = number - 1
        }
        self.paint_fall_block();
    }

}
