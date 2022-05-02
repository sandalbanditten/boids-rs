use crate::model::Model;
use crate::text::{show_current_values, show_help_menu};
use nannou::prelude::{App, Frame, Update};
use rayon::prelude::*;

// Update the state of our application every frame
pub fn update(_app: &App, model: &mut Model, _update: Update) {
    // Create a temp flock, to ensure thread safety, so that the actual flock is not getting
    // modified *and* compared to at the same time
    let temp_flock = model.flock.clone();
    model
        .flock
        .par_iter_mut()
        .for_each(|boid| boid.flock(&temp_flock, model.win_rect));
}

// Draw our stuff to the screen every frame
pub fn view(app: &App, model: &Model, frame: Frame) {
    // Draw background
    model.draw.background().rgb(0.1569, 0.1569, 0.1569);

    // Only highlight the first boid, if it exists
    if model.keybinds.highlight_first {
        if let Some(boid) = model.flock.first() {
            boid.show_perception(&model.draw, 0.025)
        }
    }

    // Show all the boids
    for boid in &model.flock {
        boid.show(&model.draw);
        if model.keybinds.highlight_all {
            boid.show_perception(&model.draw, 0.0025);
        }
    }

    // Draw the help menu
    if model.keybinds.show_help_menu {
        show_help_menu(&model.draw, model.win_rect);
    }

    // Draw the current values
    if model.keybinds.show_current_values {
        show_current_values(&model.draw, model.win_rect, model);
    }

    // Push stuff to screen
    model.draw.to_frame(app, &frame)
        .expect("Unable to draw to the frame");
}
