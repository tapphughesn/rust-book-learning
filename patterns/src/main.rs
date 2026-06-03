fn main() {
    // literals
    let x = 1;

    match x {
        1 => println!("one!"),
        2 => println!("two!"),
        3 => println!("three!"),
        _ => println!("anything"),
    }

    // shadowing

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("fifty"),
        Some(y) => println!("matched, y = {y}"),
        _ => println!("default case, x = {x:?}"),
    }

    // or

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // ranges

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // destructuring

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => println!("On neighter axis: ({x}, {y})"),
    }

    // enums

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 150, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data");
        }
        Message::Move { x, y } => {
            println!("Move by x {x} and y {y}");
        }
        Message::Write(text) => {
            println!("text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
    }

    // nested structs and enums

    enum Color {
        Rgb(i32,i32,i32),
        Hsv(i32,i32,i32),
    }

    enum Message2 {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = Message2::ChangeColor(Color::Hsv(0, 150, 255));
    match msg {
        Message2::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message2::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
        _ => (),
    }

    // Structs and Tuples

    let ((feet, inches), Point{x, y}) = ((3,10), Point {x: 3, y: -10});

    // Ignoring Values

    // underscore

    fn foo(_: i32, y: i32) {
        println!("This code only uses the y paramter: {y}");
    }

    foo(3,4);

    // nested underscore
    
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match(setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");

    let numbers = (2,4,8,16,32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Odd indexed numbers: {first}, {third}, {fifth}");
        }
    }

    // unused variables

    let _x = 5;
    let y = 10;

    let s = Some(String::from("Hello!"));

    // if let Some(_s) = s { // causes error
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{s:?}");

    // remaining parts of a value

    struct Point2 {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point2 {x: 0, y: 0, z: 0};

    match origin {
        Point2 { x, .. } => println!("x is {x}"),
    }

    match numbers {
        (first, .., last) => {
            println!("first and last numbers are {first}, {last}");
        }
    }

    // Conditionals with Match Guards

    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, got n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // using @ bindings

    enum Message3 {
        Hello {id: i32},
    }

    let msg = Message3::Hello {id: 5};

    match msg {
        Message3::Hello { id: id @ 3..=7} => {
            println!("Found an id in range: {id}");
        }
        Message3::Hello { id: 10..=12 } => {
            println!("Found an id in another range");
        }
        Message3::Hello { id } => println!("Found some other id: {id}"),
    }
}










