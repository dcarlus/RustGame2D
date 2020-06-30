#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    /// Function to generate a square.
    fn create_square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }

    /// Get the area of the current rectangle.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /// Test if the current rectangle can entirely hold another one.
    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width) && (self.height > other.height)
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 30
    } ;
    println!("The area of a {:#?} is {} square pixels.", rect1, rect1.area()) ;

    let rect2 = Rectangle {
        width: 30,
        height: 10
    } ;
    println!("{:?} contains {:?}? {}", rect1, rect2, rect1.can_hold(&rect2)) ;

    let rect3 = Rectangle::create_square(20) ;
    println!("{:?} contains {:?}? {}", rect1, rect3, rect1.can_hold(&rect3)) ;
    println!("{:?} contains {:?}? {}", rect3, rect1, rect3.can_hold(&rect1)) ;
}
