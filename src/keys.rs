use crate::boid::Boid;
use crate::flock::Flock;
use crate::model::Model;
use nannou::prelude::{random_range, App, Key, Vec2};

pub fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Plus => {
            // Add a new boid
            // Copy the first boid and add it, if there is a first boid
            if model.flock.is_empty() {
                model.flock.push(Boid::new(
                    Vec2::new(0.0, 0.0),
                    Vec2::new(random_range(-0.1, 0.1), random_range(-0.1, 0.1)),
                    app.window_rect(),
                ));
            } else {
                let mut new_boid = model.flock.first().cloned().unwrap();
                new_boid.change_position(Vec2::new(0.0, 0.0));
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
            model.flock = Flock::new_flock(app.window_rect(), model.flock.len());
        }
        Key::S => {
            model.keybinds.highlight_all = true;
        }
        // This one does not have an equivalent in key_released
        Key::D => {
            model.keybinds.highlight_all = true;
        }
        Key::Z => {
            model.keybinds.highlight_first = true;
        }
        // This one does not have an equivalent in key_released
        Key::X => {
            model.keybinds.highlight_first = true;
        }
        Key::H => {
            model.keybinds.show_help_menu = true;
        }
        // This one does not have an equivalent in key_released
        Key::J => {
            model.keybinds.show_help_menu = true;
        }
        Key::C => {
            model.keybinds.show_current_values = true;
        }
        // This one does not have an equivalent in key_released
        Key::V => {
            model.keybinds.show_current_values = true;
        }
        // The keys for modifying the boids //
        // Perception range
        Key::LBracket => {
            for boid in &mut model.flock {
                boid.change_perception(0.99);
            }
        }
        Key::RBracket => {
            for boid in &mut model.flock {
                boid.change_perception(1.01);
            }
        }
        Key::Down => {
            for boid in &mut model.flock {
                boid.change_diameter(0.99)
            }
        }
        Key::Up => {
            for boid in &mut model.flock {
                boid.change_diameter(1.01)
            }
        }
        // Max speed
        Key::Key1 => {
            for boid in &mut model.flock {
                boid.change_max_speed(0.99);
            }
        }
        Key::Key2 => {
            for boid in &mut model.flock {
                boid.change_max_speed(1.01);
            }
        }
        // Max force
        Key::Key3 => {
            for boid in &mut model.flock {
                boid.change_max_force(0.99);
            }
        }
        Key::Key4 => {
            for boid in &mut model.flock {
                boid.change_max_force(1.01);
            }
        }
        // Alignment modifier
        Key::Key5 => {
            for boid in &mut model.flock {
                boid.change_alignment_modifier(0.99);
            }
        }
        Key::Key6 => {
            for boid in &mut model.flock {
                boid.change_alignment_modifier(1.01);
            }
        }
        // Cohesion modifier
        Key::Key7 => {
            for boid in &mut model.flock {
                boid.change_cohesion_modifier(0.99);
            }
        }
        Key::Key8 => {
            for boid in &mut model.flock {
                boid.change_cohesion_modifier(1.01);
            }
        }
        // Separation modifier
        Key::Key9 => {
            for boid in &mut model.flock {
                boid.change_separation_modifier(0.99);
            }
        }
        Key::Key0 => {
            for boid in &mut model.flock {
                boid.change_separation_modifier(1.01);
            }
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
        _ => (),
    }
}

// The values read by functions in the draw function
pub struct Keybinds {
    pub highlight_all: bool,
    pub highlight_first: bool,
    pub show_help_menu: bool,
    pub show_current_values: bool,
}
