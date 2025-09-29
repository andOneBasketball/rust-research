#![allow(unused)]
fn main() {
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }

    fn read_username_from_file_easy() -> Result<String, io::Error> {
        let mut username = String::new();

        File::open("hello.txt")?.read_to_string(&mut username)?;

        Ok(username)
    }

    let mut content = read_username_from_file().unwrap();
    println!("content {:?}", content);

    content = read_username_from_file_easy().unwrap();
    println!("content {:?}", content);
}
