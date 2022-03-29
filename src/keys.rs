use crate::boid::Boid;
use crate::model::Model;
use nannou::prelude::*;

pub fn key_pressed(app: &App, model: &mut Model, key: Key) {
    // An example
    match key {
        Key::W => {
            model.flock.push(Boid::new(
                Vec2::new(0.0, 0.0),
                Vec2::new(random_range(-0.1, 0.1), random_range(-0.1, 0.1)),
                Vec2::new(0.0, 0.0),
            ));
            // Set the new window title
            app.main_window()
                .set_title(format!("{} boids!", model.flock.len()).as_str());
        }
        Key::S => {
            model.flock.pop();
            // Set the new window title
            app.main_window()
                .set_title(format!("{} boids!", model.flock.len()).as_str());
        }
        _ => (),
    }
}

pub fn key_released(_app: &App, _model: &mut Model, key: Key) {
    // An example
    match key {
        Key::W => (),
        Key::S => (),
        _ => (),
    }
}
