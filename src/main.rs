use crate::model::Model;
use nannou::prelude::*;
pub mod boid;
pub mod color;
pub mod keys;
pub mod math;
pub mod model;

fn main() {
    // Setting up the app
    nannou::app(Model::new).update(update).run();
}

// Update the state of our application every frame
fn update(app: &App, model: &mut Model, _update: Update) {
    // TODO: Still pretty bad
    let mut temp_flock = model.flock.clone();
    for boid in &mut temp_flock {
        boid.flock(&model.flock, app.window_rect());
    }
    model.flock = temp_flock;
}

// Draw our stuff to the screen every frame
fn view(app: &App, model: &Model, frame: Frame) {
    // Allows us to draw
    let draw = app.draw();

    // Draw background
    draw.background().rgb(0.1569, 0.1569, 0.1569);

    // TODO: Should also be toggleable
    // Only highlight the first boid, if he exists
    match model.flock.first() {
        Some(boid) => boid.show_perception(&draw),
        None => (),
    }

    for boid in &model.flock {
        // TODO: Should be toggleable
        // boid.show_perception(&draw);
        boid.show(&draw);
    }

    // Push stuff to screen
    draw.to_frame(app, &frame).unwrap();
}
