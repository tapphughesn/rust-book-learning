mod front_of_house;

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }
}

fn deliver_order() {}


mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        //Absolute path
        crate::front_of_house::hosting::add_to_waitlist();

        //Relative path
        // front_of_house::hosting::add_to_waitlist();

        // Now with the use statement above
        hosting::add_to_waitlist();

        // Order a breakfast in the summer with Rye toast.
        let mut meal = crate::back_of_house::Breakfast::summer("Rye");

        // Change our mind about what bread we'd like.
        meal.toast = String::from("Wheat.");
        println!("I'd like the {} toast please", meal.toast);

        // Cant do this
        // meal.seasonal_fruit = String::from("blueberries");

        let order1 = crate::back_of_house::Appetizer::Soup;
        let order2 = crate::back_of_house::Appetizer::Salad;
    }
}

use std::collections::HashMap;

fn main () {
    let mut map = HashMap::new();
    map.insert(1,2);
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
