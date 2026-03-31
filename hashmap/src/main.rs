use std::collections::HashMap;

fn main() {
    let field_name = String::from("Favorite Color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    map.entry(String::from("foobar")).or_insert(String::from("foo"));
}
