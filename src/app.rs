use rand::RngExt;

use ratatui::DefaultTerminal;

mod spawn_block;
mod down_block;
mod after_fall;
mod shop;

pub fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let mut app = App::new();

    while !app.game_over {

        while !app.game {
            // блок появляеться
            spawn_block::run(&mut app);

            // блок падает
            let _ = down_block::falling_block(&mut app, terminal);

            // проверка после падения
            let _ = after_fall::handlet_fall(&mut app);
            // увеличивает корость падения блоков посе раунда
            app.config.speed -= 5;
        }

        // увеличивает корость падения блоков посе раунда
        app.config.speed -= 50;

        // магазин
        let _ = shop::shop(&mut app, terminal);



    }
    Ok(())
}

#[derive(Debug, Clone)]
pub struct GameConfig {
    pub shop: bool,
    // нужное количество очков
    pub score_end: u32,
    // скорость падения блоков
    pub speed: u64,

    pub score_row1: u32,
    pub score_row2: u32,
    pub score_row3: u32,
    pub score_row4: u32,

    pub score_factor_row2: u32,
    pub score_factor_row3: u32,
    pub score_factor_row4: u32,
}

impl Default for GameConfig {
    fn default() -> Self {
        Self {
            shop: false,

            score_end: 100,

            speed: 1000,

            score_row1: 50,
            score_row2: 15,
            score_row3: 30,
            score_row4: 50,

            score_factor_row2: 2,
            score_factor_row3: 3,
            score_factor_row4: 5,
        }
    }
}


#[derive(Debug, Default)]
pub struct App {
    // прочие параметры игры
    pub config: GameConfig,
    // проигрыш
    pub game_over: bool,
    // игра
    pub game: bool,
    // идет ли игра
    pub falling: bool,
    // карта
    pub matrix: [[u8; 10]; 20],
    // падающий блок
    pub fall_block: [[u8; 2]; 4],
    // фигуры
    pub list_figures: [[[(i8, i8); 4]; 4]; 7],
    // мешок фигур
    pub bag_figures: Vec<[[(i8, i8); 4]; 4]>,
    // текущая фигура
    pub current_figure: [[(i8, i8); 4]; 4],
    // цвет фигур
    pub color_arr: Vec<u8>,
    pub color_block: u8,
    // поворот фигуры
    pub turn: i8,
    // очки
    pub score: u32,
}

impl App {

    pub fn new() -> Self {
        Self::default()
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

    pub fn handling_bag(&mut self) -> [[(i8, i8); 4]; 4] {
        if self.bag_figures.is_empty() {
            self.figures_generation();
            for (n, i) in (0..self.list_figures.len()).enumerate() {
                self.color_arr.push((n + 1) as u8);
                self.bag_figures.push(self.list_figures[i]);
            }
            let mut rng = rand::rng();
            for i in (1..self.bag_figures.len()).rev() {
                let j = rng.random_range(0..=i);
                self.bag_figures.swap(i, j);
                self.color_arr.swap(i, j);
            }
        }
        let current_figure = self.bag_figures.remove(0);
        let current_color = self.color_arr.remove(0);
        self.current_figure = current_figure;
        self.color_block = current_color;
        current_figure
    }
}
