fn main() {
    another_function(plus_one(five()));
    print_labeled_measurement(5, 'h');
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit: char) {
    println!("The measurement is: {value}{unit}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
