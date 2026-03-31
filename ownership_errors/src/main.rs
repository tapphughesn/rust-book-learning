fn main() {
    println!("Hello, world!");
}

fn return_a_string() -> &String {
    let s = String::from("Hello world");
    &s
}
