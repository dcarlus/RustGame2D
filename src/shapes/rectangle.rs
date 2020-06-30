#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    // Function to generate a rectangle.
    pub fn create(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height
        }
    }

    /// Function to generate a square.
    pub fn create_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }

    /// Get the area of the current rectangle.
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Test if the current rectangle can entirely hold another one.
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width) && (self.height > other.height)
    }
}
