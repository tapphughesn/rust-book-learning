fn main() {
    let v: Vec<i32> = vec![1,2,3];
    // let x: &[i32] = &*v;
    // println!("{:?}", x);
    println!("{:?}", v);
    println!("{:?}", &*v);
}

// fn first_or(strings: &Vec<String>, default: &String) -> & String {
//     if strings.len() > 0 {
//         &strings[0]
//     } else {
//         default
//     }
// }

// fn first_or<'a, 'b, 'c>(strings: &'a Vec<String>, default: &'b String) -> &'c String {
//     if strings.len() > 0 {
//         &strings[0]
//     } else {
//         default
//     }
// }
