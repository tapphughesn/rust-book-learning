use std::time::Duration;
use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        }
        else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    let store = Inventory{
        shirts: vec![
            ShirtColor::Blue,
            ShirtColor::Blue,
            ShirtColor::Red,
        ]
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // Defining a closure in a variable with annotated type
    // let expensive_closure = |num: u32| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    
    // defining a closure that captures an immutable reference
    let mut list = vec![1,2,3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    //borrow mutably
    let mut borrows_mutably = || list.push(7);
    // cant do this because list is borrowed mutably here
    // println!("Before calling closure: {list:?}");
    borrows_mutably();
    println!("After calling closure that borrows mutably: {list:?}");

    // moving into a new thread
    thread::spawn(move || println! ("From thread: {list:?}"))
        .join()
        .unwrap();
    // cant do this because list is moved into thread and dropped
    // println!("After calling closure that moves into thread: {list:?}");

}
