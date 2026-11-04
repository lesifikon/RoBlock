use crate::ui::render;
use std::time::{Duration, Instant};
use crossterm::event::{self, Event, KeyCode};
use rand::seq::SliceRandom; 

use ratatui::DefaultTerminal;

// let matrix = [[0u8; 10]; 20];

pub fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut app = App::new();
    app.figures_generation();
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
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                        break Ok(());
                    },
                    KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Left => app.left(),
                    KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down => {app.down(); last_tick = Instant::now();},
                    KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Right => app.right(),
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
}

#[derive(Debug, Default)]
pub struct App {
    // карта
    pub matrix: [[u8; 10]; 20],
    // падающий блок
    pub fall_block: [[u8; 2]; 4],
    // фигуры
    pub list_figures: [[[(i8, i8); 4]; 4]; 7],
    // мешок фигур
    pub bag_figures: Vec<[(i8, i8); 4]>,
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

    pub fn figures_generation(&mut self) {
        self.list_figures = [
            // I
            [
                [(-1, 0), (0, 0), (1, 0), (2, 0)],
                [(0, -1), (0, 0), (0, 1), (0, 2)],
                [(1, 0), (0, 0), (-1, 0), (-2, 0)],
                [(0, 1), (0, 0), (0, -1), (0, -2)],
            ],
            // O
            [
                [(0, 0), (1, 0), (0, 1), (1, 1)],
                [(0, 0), (1, 0), (0, 1), (1, 1)],
                [(0, 0), (1, 0), (0, 1), (1, 1)],
                [(0, 0), (1, 0), (0, 1), (1, 1)],
            ],
            // T
            [
                [(-1, 0), (0, 0), (1, 0), (0, 1)],
                [(0, -1), (0, 0), (0, 1), (-1, 0)],
                [(1, 0), (0, 0), (-1, 0), (0, -1)],
                [(0, 1), (0, 0), (0, -1), (1, 0)],
            ],
            // Z
            [
                [(-1, 0), (0, 0), (0, 1), (1, 1)],
                [(0, -1), (0, 0), (-1, 1), (-1, 0)],
                [(1, 0), (0, 0), (0, -1), (-1, -1)],
                [(0, 1), (0, 0), (1, -1), (1, 0)],
            ],
            // S
            [
                [(1, 0), (0, 0), (0, 1), (-1, 1)],
                [(0, 1), (0, 0), (-1, -1), (-1, 0)],
                [(-1, 0), (0, 0), (0, -1), (1, -1)],
                [(0, -1), (0, 0), (1, 1), (1, 0)],
            ],
            // L
            [
                [(0, -1), (0, 0), (0, 1), (1, 1)],
                [(1, 0), (0, 0), (-1, 0), (-1, 1)],
                [(0, 1), (0, 0), (0, -1), (-1, -1)],
                [(-1, 0), (0, 0), (1, 0), (1, -1)],
            ],
            // J
            [
                [(0, -1), (0, 0), (0, 1), (-1, 1)],
                [(1, 0), (0, 0), (-1, 0), (-1, -1)],
                [(0, 1), (0, 0), (0, -1), (1, -1)],
                [(-1, 0), (0, 0), (1, 0), (1, 1)]
            ]
        ]
    }

    pub fn handling_bag(&mut self) -> [(i8, i8); 4] {
        if self.bag_figures.is_empty() {
            self.figures_generation();
            for i in 0..self.list_figures.len() {
                self.bag_figures.push(self.list_figures[i][0]);
            }
            let mut rng = rand::rng();
            self.bag_figures.shuffle(&mut rng);
        }
        let mut current_figure = self.bag_figures.remove(0);
        current_figure
    }

    pub fn spawn_block(&mut self) {
        self.fall_block = [
            [1, 5],
            [1, 5],
            [1, 5],
            [1, 5],
        ];
        // нуливые блоки: [0][4],[0][5],[0][6],[0][7],
        let block = self.handling_bag();
        for (i,&(x, y)) in block.iter().enumerate() {
            let y1 = self.fall_block[i as usize][0] as i8;
            let x1 = self.fall_block[i as usize][1] as i8;

            self.fall_block[i as usize] = [(y1 + y) as u8, (x1 + x) as u8];
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

    pub fn handling_move(&mut self, axis:u8 , run: i8) {
        for i in 0..self.fall_block.len() {
            self.fall_block[i][axis as usize] = ((self.fall_block[i][axis as usize]) as i8 + run) as u8
        }
    }

    pub fn down(&mut self) {
        if self.block_falling() {
            self.handling_move(0, 1)
        } else {
            self.paint_fall_block();
            self.spawn_block();
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


}
