use super::App;

impl App {
    // очки
    pub fn full_row(&mut self) {
        let mut glas = true;
        for _ in 0..12 {
            let mut number = Vec::new();
            for (n, row) in self.matrix.iter().enumerate() {
                let all_positive = row.iter().all(|&x| x > 0);
                if all_positive {
                    number.push(n);
                }
            }
            if glas {
                self.give_score(number.len());
            }
            glas = false;
            loop {
                if number.is_empty() {
                    break;
                }
                while let Some(mut i) = number.pop() {
                    while i > 0 {
                        self.matrix[i] = self.matrix[i - 1];
                        i -= 1;
                    }
                }
            }
        }
    }

    pub fn give_score(&mut self, number_row: usize) {
        let mut score = 0;

        for i in 0..number_row {
            match i {
                0 => score = score + self.config.score_row1,
                1 => score = (score + self.config.score_row2) * 2,
                2 => score = (score + self.config.score_row3) * 3,
                3 => score = (score + self.config.score_row4) * 5,
                _ => (),
            };
        }

        self.score = self.score + score;
    }
}
