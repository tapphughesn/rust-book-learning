use std::rc::Rc;
use std::cell::RefCell;

struct Node {
    val: char,
    next: Option<Rc<RefCell<Node>>>,
}

fn main() {
    let a = Rc::new(RefCell::new(Node { val: 'a', next: None }));
    let b = Rc::new(RefCell::new(Node { val: 'b', next: Some(Rc::clone(&a)) }));

    a.borrow_mut().next = Some(Rc::clone(&b));

    let mut cur = Rc::clone(&a);
    for i in 0..10 {
        println!("The {}th Node has val {}", i, cur.borrow().val);
        let next = Rc::clone(cur.borrow().next.as_ref().unwrap());
        cur = next;
    }
}
