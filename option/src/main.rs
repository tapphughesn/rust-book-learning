fn main() {
    let five = Some(5);
    let six = plus_one(five);
    println!("{:?}", six);
}

fn plus_one(i: Option<i32>) -> Option<i32> {
    match i {
        None => None,
        Some(x) => Some(x+1),
    }
}
