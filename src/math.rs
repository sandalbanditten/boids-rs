use std::ops::{Add, Div, Mul, Sub};

// Map a number within a range, to within another range
pub fn map<T>(num: T, start_one: T, stop_one: T, start_two: T, stop_two: T) -> T
where
    T: Mul<Output = T> + Div<Output = T> + Add<Output = T> + Sub<Output = T> + Copy,
{
    // Return the mapped result - copy pasted from the p5js reference
    (num - start_one) / (stop_one - start_one) * (stop_two - start_two) + start_two
}
