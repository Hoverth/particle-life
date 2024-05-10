mod atom;
mod app;
mod relation;

use crate::app::{Settings, model, update};

fn main() {
    nannou::app(model).update(update).run();
}

