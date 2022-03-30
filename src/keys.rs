use crate::boid::Boid;
use crate::model::Model;
use nannou::prelude::{random_range, App, Key, Vec2};

pub fn key_pressed(app: &App, model: &mut Model, key: Key) {
    // An example
    match key {
        Key::Plus => {
            model.flock.push(Boid::new(
                Vec2::new(0.0, 0.0),
                Vec2::new(random_range(-0.1, 0.1), random_range(-0.1, 0.1)),
                Vec2::new(0.0, 0.0),
                app.window_rect(),
            ));
            // Set the new window title
            app.main_window()
                .set_title(format!("{} boids!", model.flock.len()).as_str());
        }
        Key::Minus => {
            model.flock.pop();
            // Set the new window title
            app.main_window()
                .set_title(format!("{} boids!", model.flock.len()).as_str());
        }
        Key::R => {
            let win_rect = app.window_rect();
            for boid in &mut model.flock {
                boid.change_position(Vec2::new(
                    random_range(win_rect.left(), win_rect.right()),
                    random_range(win_rect.bottom(), win_rect.top()),
                ));
                boid.change_velocity(Vec2::new(random_range(-0.1, 1.0), random_range(-0.1, 1.0)));
            }
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
