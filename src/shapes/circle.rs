#[derive(Debug)]
pub struct Circle {
    radius: f32
}

impl Circle {
    // Function to generate a circle.
    pub fn create(radius: f32) -> Circle {
        Circle {
            radius
        }
    }

    /// Get the area of the current circle.
    pub fn area(&self) -> f32 {
        std::f64::consts::PI as f32 * (self.radius * self.radius)
    }
}
