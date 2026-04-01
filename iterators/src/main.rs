use std::iter::Map;

fn main() {
    let v1 = vec![1,2,3];

    let v1_iter = v1.iter();

    // for val in v1_iter {
    //     println!("Got: {val}");
    // }

    let v2_iter = v1_iter.map(|x| x + 1);

    for val in v2_iter {
        println!("Got: {val}");
    }
}
