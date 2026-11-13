#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

// associated functions
impl Rectangle {
    // fn area(self: &Self) { .. }
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        if self.area() > other.area() {
            return true;
        }
        false
    }

    fn square(size: u32) -> Self {
        // not a method; constructor
        Self {
            width: size,
            length: size,
        }
    }
}

#[allow(unused_mut)]
fn main() {
    let scale = 2;
    let mut rectangle = Rectangle {
        width: dbg!(30 * scale),
        length: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rectangle.area()
    );

    // the following are the same
    rectangle.area(); // automatic referencing behaviour
    (&rectangle).area();

    // dbg!(rectangle);
    // =>
    // println!("{:#?}", rectangle); won't work

    // dbg!(rectangle); dbg! macro takes ownership of a variable and gives it back;
    // println! macro takes a reference by default

    // rectangle = dbg!(rectangle);
    // or
    // dbg!(&rectangle);
    // println!("{rectangle:#?}"); works fine either way

    if rectangle.width() {
        println!(
            "The rectangle has a nonzero width; it is {}",
            rectangle.width
        );
    }

    let rect2 = Rectangle {
        width: 10,
        length: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        length: 55,
    };

    println!("Can rectangle hold rect2? {}", rectangle.can_hold(&rect2));
    println!("Can rectangle hold rect3? {}", rectangle.can_hold(&rect3));

    let square = Rectangle::square(3);
    println!("square: {square:?}");
}
