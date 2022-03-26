use crate::boid::Boid;
use crate::model::Model;
use nannou::prelude::*;

pub fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    // An example
    match key {
        Key::W => {
            model.flock.push(Boid::new(
                Vec2::new(0.0, 0.0),
                Vec2::new(random_range(-0.2, 0.2), random_range(-0.2, 0.2)),
                Vec2::new(0.0, 0.0),
            ));
        }
        Key::S => {
            model.flock.pop();
        }
        _ => (),
    }
}

pub fn key_released(_app: &App, _model: &mut Model, key: Key) {
    // An example
    match key {
        Key::W => (),
        _ => (),
    }
}
