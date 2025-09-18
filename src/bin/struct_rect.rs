struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rect {
        width: 20,
        height: 30,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
