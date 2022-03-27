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
fn update(app: &App, model: &mut Model, _update: Update) {
    // TODO: This is so bad
    // Ask on a forum?
    let mut temp_flock = model.flock.clone();
    for boid in &mut temp_flock {
        boid.flock(&model.flock);
    }
    model.flock = temp_flock;

    for boid in &mut model.flock {
        boid.update(app.window_rect());
    }
}

// Draw our stuff to the screen every frame
fn view(app: &App, model: &Model, frame: Frame) {
    // Allows us to draw
    let draw = app.draw();

    // Draw background
    draw.background().rgb(0.1, 0.1, 0.1);

    for boid in &model.flock {
        boid.show_perception(&draw);
        boid.show(&draw);
    }

    // Push stuff to screen
    draw.to_frame(app, &frame).unwrap();
}
