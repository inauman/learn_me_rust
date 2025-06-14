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
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
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

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("\n--------------------------------\n");

    let sq = Rectangle::square(3);
    println!("The area of the square is {} square pixels.", sq.area());
}
