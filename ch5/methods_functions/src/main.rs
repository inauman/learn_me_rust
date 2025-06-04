struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
    fn height(&self) -> bool {
        self.height > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("Area of rectangle is {} square pixels.", rect1.area());

    println!("\n--------------------------------\n");

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    if rect1.height() {
        println!("The rectangle has a nonzero height; it is {}", rect1.height);
    }
}