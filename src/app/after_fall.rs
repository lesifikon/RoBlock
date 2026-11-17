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
                self.give_glasses(number.len());
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

    pub fn give_glasses(&mut self, number_row: usize) {
        let mut glasses = 0;
        
        for i in 0..number_row {
            match i {
                0 => glasses = glasses + 150,
                1 => glasses =glasses * 3,
                2 => glasses = glasses * 6,
                3 => glasses = glasses * 10,
                _ => (),
            };
        }

        self.glasses = self.glasses + glasses;
    }
}
