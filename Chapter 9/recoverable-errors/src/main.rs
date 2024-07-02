use core::panic;
use std::{fs::File, io::ErrorKind};

fn main() {
    let greeting_file = match File::open("hello.txt") {
        Ok(file) => file,
        // Matching on Different Errors
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    // Shortcuts for Panic on Error: unwrap and expect
    let greeting_file = File::open("ryuk.txt").unwrap();
    // With expect
    let greeting_file =
        File::open("hello.txt").expect("hello.txt should be included in this project");

    // Propagating Errors
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("user.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }

    // A Shortcut for Propagating Errors: the ? Operator

    fn read_user_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    // With chaining
    fn read_username_chained() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }

    // Even shorter
    use std::fs;
    fn read_username_shortest_version() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }

    // Where The ? Operator Can Be Used
}
