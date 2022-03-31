use crate::model::Model;
use crate::text::{show_current_values, show_help_menu};
use nannou::prelude::*;
mod boid;
mod color;
mod flock;
mod keys;
mod math;
mod model;
mod text;
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
    let win_rect = app.window_rect();

    // Draw background
    draw.background().rgb(0.1569, 0.1569, 0.1569);

    // Only highlight the first boid, if it exists
    if model.keybinds.highlight_first {
        if let Some(boid) = model.flock.first() {
            boid.show_perception(&draw)
        }
    }

    // Show all the boids
    for boid in &model.flock {
        boid.show(&draw);
        if model.keybinds.highlight_all {
            boid.show_perception(&draw);
        }
    }

    // Draw the help menu
    if model.keybinds.show_help_menu {
        show_help_menu(&draw, win_rect);
    }

    // Draw the current values
    if model.keybinds.show_current_values {
        show_current_values(&draw, win_rect, model);
    }

    // Push stuff to screen
    draw.to_frame(app, &frame).unwrap();
}
