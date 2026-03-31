#![allow(unused)]

use std::rc::Rc;

fn main() {
    // println!("Hello, world!");
    // let v: Vec<i32> = vec![1,2,3,4,5];
    // println!("hello from main: {:?}", v);
    // consume_ownership(v);

    // let v: Vec<i32> = vec![1,2,3,4,5];
    // takes_mutable_vector(v);

    // let mut v: Vec<i32> = vec![1,2,3,4,5];
    // ref_to_element_of_mut_ref(&mut v);

    // let mut name = (
    //     String::from("Ferris"),
    //     String::from("Rustacean"),
    // );
    //
    // let first = &name.0;
    //
    // name.1.push_str(", Esq.");
    // println!("{first}, {}", name.1);

    // let mut s = String::from("foo");
    // let s_ref = &mut s;
    // println!("{s}");
    // s_ref.push_str("bar");

    let my_opt = Some(String::from("blah"));
    let other_string = get_or_default(&my_opt);
    println!("{:?}", my_opt);
}

// fn return_a_stack_string() -> &String {
//     let s = String::from("blah");
//     &s
// }

// fn return_a_rc_string() -> Rc<String> {
//     let s = Rc::new(String::from("blah"));
//     Rc::clone(&s)
// }

// fn return_string_to_slot(slot: &mut String) {
//     slot.replace_range(.., "foobar");
// }

fn stringify_name_with_title(name: &Vec<String>) -> String {
    // name.push(String::from("Esq."));
    // let full = name.join(" ");
    // full

    // let mut name_clone = name.clone();
    // name_clone.push(String::from("Esq."));
    // let full = name_clone.join(" ");
    // full

    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}

fn consume_ownership(mut in_vec: Vec<i32>) {
    println!("hello from consume_ownership: {:?}", in_vec);

    for n in &mut in_vec {
        *n = *n * 2;
        // println!("{n}");
    }
    println!("hello from consume_ownership: {:?}", in_vec);
}

fn takes_mutable_vector(mut in_vec: Vec<i32>) {
    for n in &mut in_vec {
        *n = *n * 2;
    }
    println!("hello from consume_ownership: {:?}", in_vec);
}

fn ref_to_element_of_mut_ref(mutable: &mut Vec<i32>) {
    mutable.push(10); // proves mutability
    let r: &i32 = &mutable[0];
    println!("problem here?"); // at this point we have a &mut and & to the same data?
    println!("{:?}", mutable); // mutable is still usable as a immutable reference
    // mutable.push(10); // error -- mutable is not usable as a mutable reference
                      
    println!("{r}"); // here so r doesn't die
}

fn make_separator(user_str: &str) -> String {
    if user_str == "" {
        let default = "=".repeat(10);
        default
    } else {
        user_str.to_string()
    }
}

fn call_make_separator() {
    let blah: &str = &"foobar";
    let result = make_separator(blah);
}

fn get_or_default(arg: &Option<String>) -> String {
    match arg {
        None => String::new(),
        Some(s) => s.clone(),
    }
}











