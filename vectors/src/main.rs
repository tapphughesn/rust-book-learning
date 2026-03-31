fn main() {
    let mut x: Vec<i32> = Vec::new();
    x.push(5);
    x.push(6);
    x.push(7);

    let y = vec![1,2,3];
    
    let third = x[2];

    let y_third = y.get(2);

    println!("{}, {}", x[2], third);

    let mut word_vec: Vec<String> = vec![
        String::from("zero"),
        String::from("one"),
        String::from("two"),
    ];

    let word_first = &word_vec[0];

    println!("{}, {}", word_vec[0], word_first);

    // for mut s in word_vec {
    //     s.push_str("blah");
    // }

    for s in &mut word_vec {
        s.push_str("blah");
    }

    for s in &mut word_vec {
        s.push_str("blah");
    }


    println!("Hello, world!");
}
