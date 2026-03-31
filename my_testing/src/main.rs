fn main() {
    let mut x = 5;
    let mut y = 10;

    let mut a = &mut x;

    println!("a: {a}");

    a = &mut y;

    println!("a: {a}");

    x = x + 1;

    y = y + 1;

    println!("x: {x}");
    println!("y: {y}");
    //uncomment this line to get a borrow checker error
    // println!("a: {a}");
}
