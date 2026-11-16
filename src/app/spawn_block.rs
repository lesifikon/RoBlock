use super::App;

pub fn run(app: &mut App) {
    app.falling = false;
    app.figures_generation();
    app.spawn_block();
    app.lox();
    app.paint_fall_block();
}

impl App {

    pub fn loss(&mut self) {
        self.game_over = true;
        self.falling = true;
    }

    pub fn lox(&mut self) {
        for [x, y] in self.fall_block.iter() {
            if self.matrix[*x as usize][*y as usize] > 0 {
                self.game_over = true;
                self.falling = true;
            }
        }
    }


    pub fn spawn_block(&mut self) {
        // проверить не заполнен ляд
        self.turn = 0;
        self.fall_block = [
            [1, 5],
            [1, 5],
            [1, 5],
            [1, 5],
        ];
        let block = self.handling_bag();
        for (i,&(x, y)) in block[0].iter().enumerate() {
            let y1 = self.fall_block[i as usize][0] as i8;
            let x1 = self.fall_block[i as usize][1] as i8;

            self.fall_block[i as usize] = [(y1 + y) as u8, (x1 + x) as u8];
        }

    }

}
