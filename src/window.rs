use crate::model::Model;
use nannou::prelude::{App, Vec2};

pub fn window_resized(app: &App, model: &mut Model, _dim: Vec2) {
    let win_rect = app.window_rect();
    for boid in &mut model.flock {
        boid.change_boundary(win_rect);
    }
}
