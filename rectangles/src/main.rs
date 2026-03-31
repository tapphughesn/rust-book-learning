#![allow(unused)]

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        if (self.width > other.width) && (self.height > other.height) {
            true
        } else {
            false
        }
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };

    // println!("rect1 is {rect1:#?}");
    // dbg!(&rect1);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area(),
    );

    let rect2 = Rectangle{
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 50,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(20);

    println!("Can rect1 hold square? {}", rect1.can_hold(&square));

}

fn area_struct(rect: Rectangle) -> u32 {
    rect.width * rect.height
}

fn main_tuple() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area_tuple(rect1),
    );
    println!("{}", rect1.1)
}

fn area_tuple(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}

fn main_variables() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area_variables(width1, height1),
    );
}

fn area_variables(width: u32, height: u32) -> u32 {
    width * height
}
