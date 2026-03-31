fn main() {

    let v1 = vec![1,2,3];
    let mut _v2 = v1;

    let mut v: Vec<char> = vec!['a', 'b', 'c'];
    println!("{:?}", v);
    ascii_capitalize(&mut v);
    println!("{:?}", v);

    // let mut x = 1;
    // let y = &x;
    // let z = *y;
    // x += z;
    // println!("{x} {z}");

    // let mut foo: Vec<i32> = vec![0,1,2,3,4];
    // let num: &mut i32 = &mut (foo[2]);
    // println!("num is {num}");
    // println!("foo is {:?}", foo);
    // println!("num is {num}");

    // let mut x: Box<i32> = Box::new(1);
    // let a: i32 = *x;
    // *x += 1;
    //
    // let r1: &Box<i32> = &x;
    // let b: i32 = **r1;
    //
    // let r2: &i32 = &*x;
    // let c: i32 = *r2;
    //
    // println!("a, b, c: {a}, {b}, {c}");
}

fn ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0];
    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase();
        v[0] = up;
    } else {
        println!("Already capitalized: {:?}", v)
    }
}
