use crate::model::Model;
use nannou::prelude::{text, Draw, Rect};

pub fn show_help_menu(draw: &Draw, win_rect: Rect) {
    draw.text(
        " Keybinds:
  R - reset the simulation
  C - show the current values
  S - highlight perception range of all boids
  X - highlight perception range of one boids
  H - show this help menu
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
    .rgba(1.0, 1.0, 1.0, 1.0);
}

pub fn show_current_values(draw: &Draw, win_rect: Rect, model: &Model) {
    if model.flock.is_empty() {
        draw.text(
            " Current values:
  There are no boids :(
            ",
        )
        .xy(win_rect.bottom_left())
        .w_h(0.0, 0.0)
        .no_line_wrap()
        .justify(text::Justify::Left)
        .align_text_bottom()
        .font_size(1)
        .rgba(1.0, 1.0, 1.0, 1.0);
    } else {
        draw.text(
            format!(
                " Current values:
  Number of boids: {}
  Perception range: {}
  Diameter of boids: {}
  Max speed: {}
  Max force: {}
  Alignment modifier: {}
  Cohesion modifier: {}
  Separation modifier: {}",
                // Only using unwrap here, because the above if statement makes sure there will never
                // be None as flock.first()
                model.flock.len(),
                model.flock.first().unwrap().get_perception(),
                model.flock.first().unwrap().get_diameter(),
                model.flock.first().unwrap().get_max_speed(),
                model.flock.first().unwrap().get_max_force(),
                model.flock.first().unwrap().get_alignment_modifier(),
                model.flock.first().unwrap().get_cohesion_modifier(),
                model.flock.first().unwrap().get_separation_modifier(),
            )
            .as_str(),
        )
        .x_y(win_rect.left(), win_rect.bottom() + 0.5)
        .w_h(0.0, 0.0)
        .no_line_wrap()
        .justify(text::Justify::Left)
        .align_text_bottom()
        .font_size(1)
        .rgba(1.0, 1.0, 1.0, 1.0);
    };
}
