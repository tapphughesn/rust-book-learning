#![allow(unused)]

fn main() {
    // let s = "this is a str";
    // println!(s);
    // let string = String::from("this is a String");
    // println!(string);

    // Can't index strings like in other languages
    // println!("{}", s[1]);
    // println!("{}", string[2]);

    let my_string: String = String::from("blah");
    takes_string_slice(&my_string);
}

fn takes_string_slice(s: &str) {
    println!("hello from takes_string_slice");
    println!("input: {s}");
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_nick(s: &String) -> String {
    let mut ret: String = String::new();
    for c in s.chars() {
       if c == ' ' {
           ret.push(c);
       } else {
           return ret;
       }
    }
    ret
}
