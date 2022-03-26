use crate::model::Model;
use nannou::prelude::*;
pub mod keys;
pub mod model;

fn main() {
    // Setting up the app
    nannou::app(Model::new).update(update).run();
}

// Update the state of our application every frame
fn update(_app: &App, _model: &mut Model, _update: Update) {}

// Draw our stuff to the screen every frame
fn view(app: &App, _model: &Model, frame: Frame) {
    // Allows us to draw
    let draw = app.draw();

    // Draw background
    draw.background().rgb(0.2, 0.2, 0.2);

    // Push stuff to screen
    draw.to_frame(app, &frame).unwrap();
}
