fn main() {
    let b = Box::new(0 as i32);
    println!("From main: {b}");
    println!("From main: {b}");
    println!("From main: {b}");
    print_box(&b);
    println!("From main: {b}");
}

fn print_box(b: &Box<i32>) {
    println!("hello from print_box: {b}")
}
