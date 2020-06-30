#[derive(Debug)]
pub struct Triangle {
    width: u32,
    height: u32,
    right: bool
}

impl Triangle {
    // Function to generate a circle.
    pub fn create(width: u32, height: u32, right: bool) -> Triangle {
        Triangle {
            width,
            height,
            right
        }
    }

    /// Get the area of the current triangle.
    pub fn area(&self) -> f32 {
        (self.width * self.height) as f32 / 2_f32
    }

    pub fn is_right(&self) -> bool {
        self.right
    }
}
