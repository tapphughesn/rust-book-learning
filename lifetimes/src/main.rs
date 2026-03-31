const FOO: &str = "foo";

fn main() {
    // {
    //     let x = 5;
    //     r = &x; // the subject of the reference doesn't live as long as the reference
    //             // we need the reference to live shorter than the subject
    //             // The subject should live long
    // }
    //
    // let x = 5;
    // let r = &x;
    //
    // println!("r: {r}");

    let string1 = String::from("long string is long");
    // let string2 = String::from("xyz");
    let result;

    {
        let string2 = String::from("xyz");
        let result = longer(string1.as_str(), string2.as_str());
            
    }
    println!("The longest string is {result}");
}

fn longer<'a>(first: &'a str, second: &'a str) -> &'a str {
    if first.len() >= second.len() {
        first
    } else {
        second
    }
}

// const FOO: &str = "foo";
//
// fn longer2(first: &str, second: &str) -> &str {
//     &FOO
// }
