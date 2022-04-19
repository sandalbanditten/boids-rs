use crate::model::Model;
use nannou::prelude::{text, Draw, Rect};

pub fn show_help_menu(draw: &Draw, win_rect: Rect) {
    draw.text(
        "\
 Keybinds:
  H - show this help menu
  J - sticky the help menu
  C - show the current values
  V - sticky the current values
  S - highlight perception range of all boids
  D - sticky highlight perception range of all boids
  Z - highlight perception range of one boid
  X - sticky highlight perception range of one boid
  R - reset the simulation
  T - reset the position, velocity and acceleration, but nothing else
  - - remove a boid
  + - add a boid
  [ - decrease perception range
  ] - increase perception range
  ↓ - shrink boids
  ↑ - enlarge boids
  1 - decrease max speed
  2 - increase max speed
  3 - decrease max force
  4 - increase max force
  5 - decrease alignment modifier
  6 - increase alignment modifier
  7 - decrease cohesion modifier
  8 - increase cohesion modifier
  9 - decrease separation modifier
  0 - increase separation modifier",
    )
    .xy(win_rect.top_left())
    .w_h(0.0, 0.0)
    .no_line_wrap()
    .justify(text::Justify::Left)
    .align_text_top()
    .font_size(1)
    .rgba(1.0, 1.0, 1.0, 0.5);
}

pub fn show_current_values(draw: &Draw, win_rect: Rect, model: &Model) {
    let text: String = if model.flock.is_empty() {
        String::from(
            "\
 Current values:
  There are no boids :(
            ",
        )
    } else {
        // Only using unwrap here, because the above if statement makes sure there will never
        // be None as flock.first()
        let first = model.flock.first().unwrap();
        format!(
            "\
Current values:
Number of boids: {}
Perception range: {}
Diameter of boids: {}
Max speed: {}
Max force: {}
Alignment modifier: {}
Cohesion modifier: {}
Separation modifier: {}",
            // The values to be put into the string
            model.flock.len(),
            first.get_perception(),
            first.get_diameter(),
            first.get_max_speed(),
            first.get_max_force(),
            first.get_alignment_modifier(),
            first.get_cohesion_modifier(),
            first.get_separation_modifier(),
        )
    };
    draw.text(text.as_str())
        .x_y(win_rect.right() - 0.5, win_rect.bottom() + 0.5)
        .w_h(0.0, 0.0)
        .no_line_wrap()
        .justify(text::Justify::Right)
        .align_text_bottom()
        .font_size(1)
        .rgba(1.0, 1.0, 1.0, 0.5);
}
