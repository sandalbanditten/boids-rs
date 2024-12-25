use crate::boid::Boid;
use crate::flock::Flock;
use crate::model::Model;

use nannou::prelude::{random_range, App, Key, Vec2};
use rayon::prelude::*;

pub fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Plus => {
            // Add a new boid
            // Copy the first boid and add it, if there is a first boid
            // TODO: Refactor this to not use .expect() and be more graceful
            if model.flock.is_empty() {
                model.flock.push(Boid::default());
            } else {
                let mut new_boid = model
                    .flock
                    .first()
                    .copied()
                    .expect("We checked that there is a first boid");
                new_boid.change_position(Vec2::ZERO);
                new_boid
                    .change_velocity(Vec2::new(random_range(-0.1, 0.1), random_range(-0.1, 0.1)));
                model.flock.push(new_boid);
            }
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
            // Reset the boids //
            if !model.keybinds.any_is_pressed {
                model.flock = Flock::new_flock(app.window_rect(), model.flock.len());
                model.keybinds.any_is_pressed = true;
            }
        }
        Key::T => {
            // Soft reset
            if !model.keybinds.any_is_pressed {
                model.flock.par_iter_mut().for_each(|boid| {
                    boid.change_position(Vec2::new(
                        random_range(model.win_rect.left(), model.win_rect.right()),
                        random_range(model.win_rect.bottom(), model.win_rect.top()),
                    ));
                    boid.change_position(Vec2::new(
                        random_range(model.win_rect.left(), model.win_rect.right()),
                        random_range(model.win_rect.bottom(), model.win_rect.top()),
                    ));
                });
                model.keybinds.any_is_pressed = true;
            }
        }
        Key::S => {
            model.keybinds.highlight_all = true;
        }
        Key::D => {
            if !model.keybinds.any_is_pressed {
                model.keybinds.highlight_all = !model.keybinds.highlight_all;
                model.keybinds.any_is_pressed = true;
            }
        }
        Key::Z => {
            model.keybinds.highlight_first = true;
        }
        Key::X => {
            if !model.keybinds.any_is_pressed {
                model.keybinds.highlight_first = !model.keybinds.highlight_first;
                model.keybinds.any_is_pressed = true;
            }
        }
        Key::H => {
            model.keybinds.show_help_menu = true;
        }
        Key::J => {
            if !model.keybinds.any_is_pressed {
                model.keybinds.show_help_menu = !model.keybinds.show_help_menu;
                model.keybinds.any_is_pressed = true;
            }
        }
        Key::C => {
            model.keybinds.show_current_values = true;
        }
        Key::V => {
            if !model.keybinds.any_is_pressed {
                model.keybinds.show_current_values = !model.keybinds.show_current_values;
                model.keybinds.any_is_pressed = true;
            }
        }
        // The keys for modifying the boids //
        // Perception range
        Key::LBracket => {
            model.flock.par_iter_mut().for_each(|boid| {
                boid.change_perception(0.99);
            });
        }
        Key::RBracket => {
            model.flock.par_iter_mut().for_each(|boid| {
                boid.change_perception(1.01);
            });
        }
        Key::Down => {
            model
                .flock
                .par_iter_mut()
                .for_each(|boid| boid.change_diameter(0.99));
        }
        Key::Up => {
            model
                .flock
                .par_iter_mut()
                .for_each(|boid| boid.change_diameter(1.01));
        }
        // Max speed
        Key::Key1 => {
            model.flock.par_iter_mut().for_each(|boid| {
                boid.change_max_speed(0.99);
            });
        }
        Key::Key2 => {
            model.flock.par_iter_mut().for_each(|boid| {
                boid.change_max_speed(1.01);
            });
        }
        // Max force
        Key::Key3 => {
            model.flock.par_iter_mut().for_each(|boid| {
                boid.change_max_force(0.99);
            });
        }
        Key::Key4 => {
            model.flock.par_iter_mut().for_each(|boid| {
                boid.change_max_force(1.01);
            });
        }
        // Alignment modifier
        Key::Key5 => {
            model.flock.par_iter_mut().for_each(|boid| {
                boid.change_alignment_modifier(0.99);
            });
        }
        Key::Key6 => {
            model.flock.par_iter_mut().for_each(|boid| {
                boid.change_alignment_modifier(1.01);
            });
        }
        // Cohesion modifier
        Key::Key7 => {
            model.flock.par_iter_mut().for_each(|boid| {
                boid.change_cohesion_modifier(0.99);
            });
        }
        Key::Key8 => {
            model.flock.par_iter_mut().for_each(|boid| {
                boid.change_cohesion_modifier(1.01);
            });
        }
        // Separation modifier
        Key::Key9 => {
            model.flock.par_iter_mut().for_each(|boid| {
                boid.change_separation_modifier(0.99);
            });
        }
        Key::Key0 => {
            model.flock.par_iter_mut().for_each(|boid| {
                boid.change_separation_modifier(1.01);
            });
        }
        _ => (),
    }
}

pub fn key_released(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => {
            model.keybinds.highlight_all = false;
        }
        Key::Z => {
            model.keybinds.highlight_first = false;
        }
        Key::H => {
            model.keybinds.show_help_menu = false;
        }
        Key::C => {
            model.keybinds.show_current_values = false;
        }
        _ => {
            model.keybinds.any_is_pressed = false;
        }
    }
}

// The values read by functions in the draw function
pub struct Keybinds {
    pub highlight_all: bool,
    pub highlight_first: bool,
    pub show_help_menu: bool,
    pub show_current_values: bool,
    // The is_pressed is for preventing the behavior that holding down a key repeatedly creates
    pub any_is_pressed: bool,
}

impl Default for Keybinds {
    fn default() -> Self {
        Self {
            highlight_all: false,
            highlight_first: false,
            show_help_menu: true,
            show_current_values: false,
            any_is_pressed: false,
        }
    }
}
