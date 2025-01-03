use crate::model::Model;

use nannou::prelude::{text, Draw, Rect, Vec2};

const FONT_SIZE: u32 = 22;

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
    .wh(Vec2::ZERO)
    .no_line_wrap()
    .justify(text::Justify::Left)
    .align_text_top()
    .font_size(FONT_SIZE)
    .rgba(1.0, 1.0, 1.0, 0.5);
}

pub fn show_current_values(draw: &Draw, win_rect: Rect, model: &Model) {
    let text = if let Some(first) = model.flock.first() {
        format!(
            "\
Current values:
Number of boids: {}
Perception radius: {}
Diameter of boids: {}
Max speed: {}
Max force: {}
Alignment modifier: {}
Cohesion modifier: {}
Separation modifier: {}",
            // The values to be put into the string
            model.flock.len(),
            first.perception_radius(),
            first.diameter(),
            first.max_speed(),
            first.max_force(),
            first.alignment_modifier(),
            first.cohesion_modifier(),
            first.separation_modifier(),
        )
    } else {
        String::from(
            "\
 Current values:
  There are no boids :(
            ",
        )
    };
    draw.text(text.as_str())
        .x_y(win_rect.left() + 5.0, win_rect.bottom() + 5.0)
        .wh(Vec2::ZERO)
        .no_line_wrap()
        .justify(text::Justify::Left)
        .align_text_bottom()
        .font_size(FONT_SIZE)
        .rgba(1.0, 1.0, 1.0, 0.5);
}
