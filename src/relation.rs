pub struct Relation {
    pub table: Vec<Vec<f32>>,
}

impl Relation {
    pub fn new(s: usize) -> Self {
        Self {
            table: vec![vec![0.0; s]; s],
        }
    }

    pub fn get(&self, c: usize, r: usize) -> f32 {
        self.table[c][r]
    }
    
    pub fn set(&mut self, v: f32, c: usize, r: usize) {
        self.table[c][r] = v;
    }
}
