enum Shape {
    Circle(f64),          // radius
    Rectangle(f64, f64),  // width, height
}

fn main() {
    let rect = Shape::Rectangle(10.0, 20.0);
    let circle = Shape::Circle(15.0);

    println!("Area of rectangle: {}", shape_area(rect));
    println!("Area of circle: {}", shape_area(circle));
}

fn shape_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
    }
}