mod shapes ;

#[derive(Debug)]
enum Shape {
    Rectangle(shapes::Rectangle),
    Triangle(shapes::Triangle),
    Circle(shapes::Circle)
}

impl Shape {
    fn print(&self) {
        match self {
            Shape::Rectangle(shape) => println!(
                "The rectangle area is {} pixel^2",
                shape.area()
            ),
            Shape::Triangle(shape) => println!(
                "The {} area is {} pixel^2",
                if shape.is_right() { "right triangle" } else { "triangle" },
                shape.area()
            ),
            Shape::Circle(shape) => println!(
                "The circle area is {} pixel^2",
                shape.area()
            ),
            _ => println!("This is an unknown shape...")
        }
    }
}

fn main() {
    let testRect: Shape = Shape::Rectangle(shapes::Rectangle::create(78, 42)) ;
    testRect.print() ;

    let testCircle: Shape = Shape::Circle(shapes::Circle::create(5_f32)) ;
    testCircle.print() ;

    let testRightTriangle: Shape = Shape::Triangle(shapes::Triangle::create(12, 8, true)) ;
    testRightTriangle.print() ;

    let testTriangle: Shape = Shape::Triangle(shapes::Triangle::create(12, 8, false)) ;
    testTriangle.print() ;
}
