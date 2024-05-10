mod atom;
mod app;

use crate::app::{Settings, model, update};

fn main() {
    nannou::app(model).update(update).run();
}





struct Relation {
    table: Vec<Vec<f32>>,
}
impl Relation {
    fn new(s: usize) -> Self {
        Self {
            table: vec![vec![0.0; s]; s],
        }
    }
    fn get(&self, c: usize, r: usize) -> f32 {
        self.table[c][r]
    }
    fn set(&mut self, v: f32, c: usize, r: usize) {
        self.table[c][r] = v;
    }
}

