struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn debug() -> u32 {
        1
    }
}

fn main() {
    let rect1 = Rect {
        width: 20,
        height: 30,
    };
    // Calling the instance methods
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "The perimeter of the rectangle is {} pixels.",
        rect1.perimeter()
    );
    // Calling the static method
    println!("The debug value is {}.", Rect::debug());
}
