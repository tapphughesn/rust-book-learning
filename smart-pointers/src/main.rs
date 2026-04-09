use crate::List::{Cons, Nil};
use std::{ops::Deref, rc::Rc};
use std::cell::RefCell;

fn main() {
    let b = Box::new(5);
    println!("b = {b}");

    // let _list = Cons(1, Box::new(Cons( 2, Box::new(Cons( 3, Box::new(Nil))))));

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

    // reference counting with Rc<T>
    // println!("Doing reference counting!");
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("count after creating a = {}", Rc::strong_count(&a));
    // let b = Cons(3, Rc::clone(&a));
    // println!("count after creating b = {}", Rc::strong_count(&a));
    // {
    //     let c = Cons(3, Rc::clone(&a));
    //     println!("count after creating c = {}", Rc::strong_count(&a));
    // }
    // println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // Making the list mutable by introducing RefCell
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;
    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
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
