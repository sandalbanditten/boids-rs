use crate::model::Model;
use nannou::prelude::*;

pub fn key_pressed(_app: &App, _model: &mut Model, key: Key) {
    // An example
    match key {
        Key::W => (),
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
