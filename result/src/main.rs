use std::fs::File;
use std::io::{self, Read};
use std::io::ErrorKind;

fn main() {
    let greeting_file_result: Result<File, std::io::Error> = File::open("hello.txt");

    // let greeting_file = match greeting_file_result{
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(error) => panic!("Problem creating the file: {error:?}"),
    //         },
    //         _ => panic!("Problem opening the file. Error: {error:?}")
    //
    //     },
    // };

    // let greeting_file = greeting_file_result.unwrap();

    let greeting_file = greeting_file_result
        .expect("hello.txt should be included in this project");
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let username_file_result = File::open("hello.txt");
    // let mut username_file = username_file_result?;
    //
    // let mut username = String::new();
    //
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?
}
