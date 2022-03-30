// Map a number within a range, to within another range
pub fn map(num: f32, start_one: f32, stop_one: f32, start_two: f32, stop_two: f32) -> f32 {
    // Return the mapped result - copy pasted from the p5js reference
    (num - start_one) / (stop_one - start_one) * (stop_two - start_two) + start_two
}
