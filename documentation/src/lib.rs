//! # My Documentation Crate
//! 
//! `documentation` is a crate designed only to test cargo's documentation abilities.
//! It provides some other functions, however, you'd be a fool to use them.

/// Adds one to the number given
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = documentation::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

/// Returns a greeting, given your name
///
/// # Examples
/// ```
/// let my_name = String::from("Nick");
/// let greeting = documentation::greet(&my_name);
///
/// assert_eq!(String::from("Hello Nick"), greeting)
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello {}", name)
}
