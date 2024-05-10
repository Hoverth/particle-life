use nannou::rand::random_range;

pub struct Relation {
    pub table: Vec<Vec<f32>>,
}

impl Relation {
    pub fn new(s: usize) -> Self {
        Self {
            table: {
                let mut vec: Vec<Vec<f32>> = vec![vec![0.0; s]; s];
                for i in 0..vec.capacity() {
                    for j in 0..vec[i].capacity() {
                        vec[i][j] = ((random_range(-0.1, 0.1) * 1000.0) as f32).round() / 1000.0;
                    }
                };
                vec
            }
        }
    }

    pub fn get(&self, c: usize, r: usize) -> f32 {
        self.table[c][r]
    }
   
    #[allow(dead_code)]
    pub fn set(&mut self, v: f32, c: usize, r: usize) {
        self.table[c][r] = v;
    }
}
