use crate::model::Model;
use nannou::prelude::*;
mod boid;
mod color;
mod keys;
mod math;
mod model;
mod window;

fn main() {
    // Setting up the app
    nannou::app(Model::new).update(update).run();
}

// Update the state of our application every frame
fn update(_app: &App, model: &mut Model, _update: Update) {
    // TODO: Still pretty bad
    let mut temp_flock = model.flock.clone();
    for boid in &mut temp_flock {
        boid.flock(&model.flock);
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
    if model.highlight_first {
        if let Some(boid) = model.flock.first() {
            boid.show_perception(&draw)
        }
    }

    for boid in &model.flock {
        // TODO: Should be toggleable
        if model.highlight_all {
            boid.show_perception(&draw);
        }
        boid.show(&draw);
    }

    // Push stuff to screen
    draw.to_frame(app, &frame).unwrap();
}
