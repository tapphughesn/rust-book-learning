use crate::List::{Cons, Nil};
use std::ops::Deref;

fn main() {
    let b = Box::new(5);
    println!("b = {b}");

    let _list = Cons(1, Box::new(Cons( 2, Box::new(Cons( 3, Box::new(Nil))))));

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    //Strings have pointer-like behavior?
    let s = String::from("foo");
    let t: &str = &*s;
    let ss: &str = "string slice";

    println!("{}", s);
    println!("{}", t);

    // Deref coercion
    let m = &MyBox::new(String::from("Nick"));
    test_deref_coercion(m);

    // drop
    let c = CustomSmartPointer {
        data: String::from("first csp"),
    };
    let d = CustomSmartPointer {
        data: String::from("second csp"),
    };
    // d.drop(); // Not allowed
    drop(d);
    // println!("d's data: {}", d.data); // Not allowed after drop (move)
    println!("CustomSmartPointers created!");
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn test_deref_coercion(name: &str) {
    println!("Hello {name}");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`", self.data);
    }
}
