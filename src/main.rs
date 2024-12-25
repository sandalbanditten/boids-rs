use model::Model;
use update::{update, view};

mod boid;
mod color;
mod flock;
mod keys;
mod math;
mod model;
mod text;
mod update;
mod window;

fn main() {
    // Setting up the app
    nannou::app(Model::new).update(update).run();
}
