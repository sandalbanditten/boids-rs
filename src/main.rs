use crate::model::Model;
use nannou::prelude::*;
pub mod boid;
pub mod color;
pub mod keys;
pub mod model;

fn main() {
    // Setting up the app
    nannou::app(Model::new).update(update).run();
}

// Update the state of our application every frame
fn update(_app: &App, model: &mut Model, _update: Update) {
    for boid in &mut model.flock {
        boid.update();
    }
}

// Draw our stuff to the screen every frame
fn view(app: &App, model: &Model, frame: Frame) {
    // Allows us to draw
    let draw = app.draw();

    // Draw background
    draw.background().rgb(0.0, 0.0, 0.0);

    for boid in &model.flock {
        boid.show(&draw);
    }

    // Push stuff to screen
    draw.to_frame(app, &frame).unwrap();
}
