use crate::model::Model;
use nannou::prelude::{App, Vec2};

pub fn window_resized(app: &App, model: &mut Model, _dim: Vec2) {
    model.win_rect = app.window_rect();
}
